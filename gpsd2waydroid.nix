{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "gpsd2waydroid";
  version = "0.1.0";

  src = pkgs.fetchFromGitHub {
    owner = "vblimits";
    repo = "gpsd2waydroid";
    rev = "master";
    sha256 = "vITOvF14+nsyBYvSi5ce0BfQ5TLYRYGZlT9msnHJlpA="; # Placeholder, replace with actual hash
  };

  cargoSha256 = "pdS8flxSwhuGgojlqCQVPobYVY8ffx9MsO4J122kTfQ="; # Placeholder, replace with actual hash
}