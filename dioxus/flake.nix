{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }:
  let
    supportedSystems = nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ];
  in
  {
    devShells = supportedSystems (system: {
      default = with nixpkgs.legacyPackages.${system}; mkShell {
        packages = [
          cargo
          dioxus-cli
          lld
          rustc
          wasm-bindgen-cli_0_2_126
        ];
      };
    });
  };
}
