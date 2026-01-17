{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    bzip2
  ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
    alsa-lib
    alsa-plugins
    udev
  ];

  shellHook = ''
    export PATH="$HOME/.nix-profile/bin:$PATH"
  '';
}
