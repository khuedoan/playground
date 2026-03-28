{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs = { nixpkgs, ... }:
  let
    supportedSystems = nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    linuxSystems = nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
    ];
    mkJarvis = system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      pkgs.rustPlatform.buildRustPackage {
        pname = "jarvis";
        version = "0.1.0";
        src = ./.;
        useFetchCargoVendor = true;
        # Run `nix build` once — the error will show the correct hash to paste here
        cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl ];
      };
  in
  {
    packages = supportedSystems (system: rec {
      jarvis = mkJarvis system;
      default = jarvis;
    });

    # Docker image (Linux only)
    dockerImages = linuxSystems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        jarvis = mkJarvis system;
      in {
        default = pkgs.dockerTools.buildLayeredImage {
          name = "jarvis";
          tag = "latest";
          contents = [
            jarvis
            pkgs.coreutils
            pkgs.gnugrep
            pkgs.findutils
            pkgs.diffutils
            pkgs.git
            pkgs.cacert
          ];
          config = {
            Entrypoint = [ "${jarvis}/bin/jarvis" ];
            Env = [
              "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            ];
          };
        };
      }
    );

    devShells = supportedSystems (system: {
      default = with nixpkgs.legacyPackages.${system}; mkShell {
        packages = [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          pkg-config
          openssl
        ];
      };
    });
  };
}
