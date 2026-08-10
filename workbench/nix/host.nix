{ self, microvm }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.workbench;
  hostAgent = self.packages.${pkgs.system}.host-agent;
  microvmCommand = microvm.packages.${pkgs.system}.microvm;
in
{
  imports = [ microvm.nixosModules.host ];

  options.services.workbench = {
    enable = lib.mkEnableOption "the Workbench MicroVM host agent";

    listenAddress = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1:9090";
      description = "Address for the private host-agent API.";
    };

    bridgeName = lib.mkOption {
      type = lib.types.str;
      default = "workbench0";
      description = "Bridge used by workspace TAP interfaces.";
    };

    environmentFile = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Root-only environment file containing model credentials and Pi selection.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ microvmCommand ];

    boot.kernel.sysctl."net.ipv4.ip_forward" = 1;
    networking.useNetworkd = true;
    systemd.network.enable = true;
    systemd.network.netdevs."20-workbench".netdevConfig = {
      Kind = "bridge";
      Name = cfg.bridgeName;
    };
    systemd.network.networks = {
      "20-workbench-bridge" = {
        matchConfig.Name = cfg.bridgeName;
        address = [ "10.88.0.1/16" ];
        networkConfig = {
          ConfigureWithoutCarrier = true;
          IPMasquerade = "ipv4";
        };
      };
      "20-workbench-taps" = {
        matchConfig.Name = "wb-*";
        networkConfig.Bridge = cfg.bridgeName;
      };
    };
    services.resolved = {
      enable = true;
      extraConfig = "DNSStubListenerExtra=10.88.0.1";
    };
    networking.firewall.interfaces.${cfg.bridgeName} = {
      allowedTCPPorts = [ 53 ];
      allowedUDPPorts = [ 53 ];
    };

    systemd.tmpfiles.rules = [
      "d /var/lib/workbench 0700 root root -"
      "d /var/lib/workbench/specs 0700 root root -"
      "d /run/workbench/credentials 0700 root root -"
    ];

    systemd.services.workbench-host-agent = {
      description = "Workbench MicroVM host agent";
      wantedBy = [ "multi-user.target" ];
      after = [
        "network-online.target"
        "systemd-networkd.service"
      ];
      wants = [ "network-online.target" ];
      environment = {
        RUST_LOG = "info";
        WORKBENCH_HOST_LISTEN = cfg.listenAddress;
        WORKBENCH_HOST_STATE = "/var/lib/workbench/state.json";
        WORKBENCH_MICROVM = "${microvmCommand}/bin/microvm";
        WORKBENCH_SYSTEMCTL = "${pkgs.systemd}/bin/systemctl";
        WORKBENCH_FLAKE_ROOT = toString self;
        WORKBENCH_SPEC_ROOT = "/var/lib/workbench/specs";
        WORKBENCH_MICROVM_STATE_ROOT = "/var/lib/microvms";
        WORKBENCH_CREDENTIAL_ROOT = "/run/workbench/credentials";
      };
      serviceConfig = {
        ExecStart = "${hostAgent}/bin/workbench-host-agent";
        EnvironmentFile = lib.optional (cfg.environmentFile != null) cfg.environmentFile;
        Restart = "on-failure";
        RestartSec = 2;
        User = "root";
      };
    };
  };
}
