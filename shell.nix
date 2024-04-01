{...}:
let

  pkgs = import (fetchTarball("https://github.com/nixos/nixpkgs/tarball/master")) {};

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = [ 
      pkgs.cargo 
      pkgs.rustc 
      pkgs.pkg-config 
      pkgs.openssl
      pkgs.hyperfine
      pkgs.memcached
      pkgs.redis
#      pkgs.rust-analyzer

    ];
}
