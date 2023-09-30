# https://status.nixos.org (nixos-22.11)
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/5cfafa12d573.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    geckodriver
    python311Packages.selenium
  ];
}
