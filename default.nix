{
    pkgs ? import <nixpkgs> {},

}:
pkgs.rustPlatform.buildRustPackage rec {
  name = "cmd_cachier";
  version = "0.3.1";

    src = fetchGit {
        url = "https://gitlab.com/inyourfoss/cmd_cachier.git";
        rev = "88010f83f56f6520aabcd6da7bd354c79ed0fb44";
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

