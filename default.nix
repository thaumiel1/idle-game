# default.nix
{ lib, fetchFromGitHub, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "idle-game";
  version = "0.0.1";

  src = ./.;

  cargoHash = "";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ systemd ];
}
