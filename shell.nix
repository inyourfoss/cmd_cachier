{...}:
let
  # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
  pkgs = import (fetchTarball("https://github.com/nixos/nixpkgs/tarball/master")) {};

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = [ 
      pkgs.cargo 
      pkgs.rustc 
      pkgs.pkg-config 
      pkgs.openssl
      pkgs.redis
      pkgs.iconv
      pkgs.asciidoctor
    ];
}
