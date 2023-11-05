{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system};
      {
        devShells.default = mkShell {
          packages = [
            delve
            go
            gopls
            sqlite
            (nodePackages.tailwindcss.overrideAttrs (attrs: {
              plugins = [
                nodePackages."@tailwindcss/typography"
              ];
            }))
          ];
        };
      }
    );
}
