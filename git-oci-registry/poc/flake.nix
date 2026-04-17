{
  description = "Git-OCI-Registry Hybrid PoC dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Container tools
            docker
            docker-compose

            # Kubernetes
            kubectl
            fluxcd
            k9s

            # Go (for building cicd binary locally)
            go

            # OCI tools
            crane
            oras

            # Utilities
            jq
            curl
          ];

          shellHook = ''
            echo "git-oci-registry PoC dev shell"
            echo "Run 'make up' to start, 'make init' to bootstrap, 'make demo' to run the demo"
          '';
        };
      }
    );
}
