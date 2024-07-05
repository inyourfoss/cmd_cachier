{
    pkgs ? import <nixpkgs> {},

}:
pkgs.rustPlatform.buildRustPackage rec {
  name = "cmd_cachier";
  version = "0.3.6";

    src = fetchGit {
        url = "https://github.com/inyourfoss/cmd_cachier.git";
        rev = "7b3d1437800f5d7d189345fa823d6fda58c8726b";
    };

  cargoLock = {
    # Needs to be in build directory
    lockFile = ./Cargo.lock;
  };

  postPatch = ''
  ln -s ${./Cargo.lock} Cargo.lock
'';
    
  nativeBuildInputs = [ pkgs.installShellFiles ];
  #buildInputs = [ ];

#  installPhase = ''
#    mkdir -p $out/usr/share/man/man1
#    cp $src/man/$name.1 $out/usr/share/man/man1/$name.1
#  '';

    postInstall = ''
        mkdir -p $out/man/man1
        cp $src/man/man1/$name.1 $out/man/man1/
    '';
}

