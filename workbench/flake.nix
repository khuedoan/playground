{
  description = "Private coding workspaces backed by microvm.nix";

  nixConfig = {
    extra-substituters = [ "https://microvm.cachix.org" ];
    extra-trusted-public-keys = [
      "microvm.cachix.org-1:oXnBc6hRE3eX5rSYdRyMYXnfzcCxC7yKPTbZXALsqys="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    microvm = {
      url = "github:astro/microvm.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      microvm,
      ...
    }:
    let
      systems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;

      rustPackage =
        pkgs: crate: binary:
        pkgs.rustPlatform.buildRustPackage {
          pname = binary;
          version = "0.1.0";
          src = nixpkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "-p"
            crate
          ];
          cargoTestFlags = [
            "-p"
            crate
          ];
          installPhase = ''
            runHook preInstall
            install -Dm755 \
              target/${pkgs.stdenv.hostPlatform.rust.rustcTarget}/release/${binary} \
              $out/bin/${binary}
            runHook postInstall
          '';
        };

      mkWorkspace =
        {
          workspaceName,
          workspaceId,
          vcpus,
          memoryMib,
          diskGiB,
          gui,
          address,
          gateway,
          mac,
          tapInterface,
          system ? "x86_64-linux",
        }:
        nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            microvm.nixosModules.microvm
            (import ./nix/guest.nix { inherit self; })
            {
              networking.hostName = workspaceName;
              workbench = {
                inherit workspaceId;
                gui.enable = gui;
              };

              microvm = {
                hypervisor = "cloud-hypervisor";
                mem = memoryMib;
                vcpu = vcpus;
                socket = "control.sock";
                interfaces = [
                  {
                    type = "tap";
                    id = tapInterface;
                    inherit mac;
                  }
                ];
                shares = [
                  {
                    proto = "virtiofs";
                    tag = "ro-store";
                    source = "/nix/store";
                    mountPoint = "/nix/.ro-store";
                    readOnly = true;
                  }
                ];
                volumes = [
                  {
                    image = "workspace.img";
                    mountPoint = "/workspace";
                    size = diskGiB * 1024;
                  }
                ];
              };

              systemd.network.networks."20-workbench" = {
                matchConfig.Type = "ether";
                address = [ "${address}/16" ];
                routes = [ { Gateway = gateway; } ];
                networkConfig.DNS = [ gateway ];
              };
            }
          ];
        };
    in
    {
      lib.mkWorkspace = mkWorkspace;

      nixosModules.host = import ./nix/host.nix { inherit self microvm; };

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          host-agent = rustPackage pkgs "workbench-host-agent" "workbench-host-agent";
          guest-agent = rustPackage pkgs "workbench-guest-agent" "workbench-guest-agent";
          microvm = microvm.packages.${system}.microvm;
          default = self.packages.${system}.host-agent;
        }
      );

      apps = forAllSystems (system: {
        host-agent = {
          type = "app";
          program = "${self.packages.${system}.host-agent}/bin/workbench-host-agent";
        };
        default = self.apps.${system}.host-agent;
      });

      checks = forAllSystems (
        system:
        let
          workspace = mkWorkspace {
            inherit system;
            workspaceName = "workbench-check";
            workspaceId = "00000000-0000-0000-0000-000000000042";
            vcpus = 2;
            memoryMib = 2048;
            diskGiB = 8;
            gui = true;
            address = "10.88.0.42";
            gateway = "10.88.0.1";
            mac = "02:b0:00:00:00:42";
            tapInterface = "wb-check";
          };
        in
        {
          inherit (self.packages.${system}) host-agent guest-agent;
          workspace-runner = workspace.config.microvm.declaredRunner;
        }
      );

      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          beam = pkgs.beam.packages.erlang_28;
        in
        {
          default = pkgs.mkShell {
            packages = [
              beam.elixir_1_19
              beam.erlang
              microvm.packages.${system}.microvm
              pkgs.cargo
              pkgs.clippy
              pkgs.git
              pkgs.nixfmt-rfc-style
              pkgs.nodejs_24
              pkgs.postgresql_17
              pkgs.rustc
              pkgs.rustfmt
              pkgs.shellcheck
            ];
          };
        }
      );
    };
}
