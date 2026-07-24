{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }:
    let
      systems = [ "aarch64-darwin" "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          topcoat-cli = pkgs.rustPlatform.buildRustPackage rec {
            pname = "topcoat-cli";
            version = "0.4.0";
            src = pkgs.fetchCrate {
              inherit pname version;
              hash = "sha256-OK15uYQRfTzvyPvLZhYFagzcGKQt+unMUlfob0vjSCo=";
            };
            cargoHash = "sha256-CyJMhjr0ucd9B/4QEZxEgxvuY1bHiQ7/q151r83ZZYg=";
            doCheck = false;
          };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          topcoat-cli = self.packages.${system}.topcoat-cli;
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.rustc
              pkgs.clippy
              pkgs.rustfmt
              topcoat-cli
            ];
          };
        }
      );
    };
}
