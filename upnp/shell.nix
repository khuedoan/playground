# https://status.nixos.org
{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/f034b5693a26625f56068af983ed7727a60b5f8b.tar.gz") {} }:

let
  python-packages = pkgs.python3.withPackages (p: with p; [
    miniupnpc
  ]);
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    python-packages
  ];
}
