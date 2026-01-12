{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    alsa-lib
    alsa-plugins
    udev
  ];

  shellHook = ''
    export PATH="$HOME/.nix-profile/bin:$PATH"
  '';
}
