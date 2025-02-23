use flexi_logger::Logger;
use futures::{future::ready, prelude::*};
use gpsd_proto::{UnifiedResponse, ENABLE_WATCH_CMD, PROTO_MAJOR_MIN};
use log::{debug, error, info, trace};
use serde_json;
use std::error::Error;
use std::net::SocketAddr;
use subprocess::Exec;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    Logger::try_with_str("info")?
        .start()?;
    info!("Starting");

    let addr: SocketAddr = "127.0.0.1:2947".parse().unwrap();

    let stream = TcpStream::connect(&addr).await?;
    let mut framed = Framed::new(stream, LinesCodec::new());

    framed.send(ENABLE_WATCH_CMD).await?;
    framed
        .try_for_each(|line| {
            trace!("Raw {line}");

            match serde_json::from_str(&line) {
                Ok(rd) => match rd {
                    UnifiedResponse::Version(v) => {
                        if v.proto_major < PROTO_MAJOR_MIN {
                            panic!("Gpsd major version mismatch");
                        }
                        info!("Gpsd version {} connected", v.rev);
                    }
                    UnifiedResponse::Devices(_) => {}
                    UnifiedResponse::Watch(_) => {}
                    UnifiedResponse::Device(d) => debug!("Device {d:?}"),
                    UnifiedResponse::Tpv(t) => {
                        debug!("Tpv {t:?}");
                        if let Some(lat) = t.lat {
                            if let Some(lon) = t.lon {
                                if let Some(speed) = t.speed {
                                    if let Some(bearing) = t.track {
                                        if let Some(alt) = t.alt {
                                            if let Some(acc) = t.eph {
                                                let command = format!(
                                                    "adb shell am start-foreground-service --user 0 -n io.appium.settings/.LocationService --es longitude {} --es latitude {} --es speed {} --es bearing {} --es altitude {} --es accuracy {}", lon, lat, speed, bearing, alt, acc
                                                );
                                                if let Err(e) = Exec::shell(command).join() {
                                                    eprintln!("Failed to forward GPS data: {}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    UnifiedResponse::Sky(s) => debug!("Sky {s:?}"),
                    UnifiedResponse::Pps(p) => debug!("PPS {p:?}"),
                    UnifiedResponse::Gst(g) => debug!("GST {g:?}"),
                },
                Err(e) => {
                    error!("Error decoding: {e}");
                }
            };

            ready(Ok(()))
        })
        .await?;

    Ok(())
}