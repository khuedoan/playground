{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.workbench;
  guestAgent = self.packages.${pkgs.system}.guest-agent;
  piModels = pkgs.writeText "workbench-pi-models.json" (
    builtins.toJSON {
      providers.github-models = {
        baseUrl = "https://models.github.ai/inference";
        api = "openai-completions";
        apiKey = "$GITHUB_MODELS_TOKEN";
        authHeader = true;
        compat = {
          supportsDeveloperRole = false;
          supportsReasoningEffort = false;
        };
        models = [
          {
            id = "openai/gpt-4.1-mini";
            name = "GitHub Models GPT-4.1 Mini";
            reasoning = false;
            input = [ "text" ];
            contextWindow = 128000;
            maxTokens = 4096;
          }
        ];
      };
    }
  );
  swayConfig = pkgs.writeText "workbench-sway.conf" ''
    output HEADLESS-1 resolution 1280x720
    default_border pixel 2
    focus_follows_mouse no
    exec ${pkgs.foot}/bin/foot
    exec ${pkgs.blender}/bin/blender
  '';
  startWayvnc = pkgs.writeShellScript "workbench-wayvnc" ''
    set -eu
    for attempt in $(${pkgs.coreutils}/bin/seq 1 100); do
      if [ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]; then
        exec ${pkgs.wayvnc}/bin/wayvnc 0.0.0.0 5900
      fi
      ${pkgs.coreutils}/bin/sleep 0.1
    done
    echo "Sway did not create $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" >&2
    exit 1
  '';
in
{
  options.workbench = {
    workspaceId = lib.mkOption {
      type = lib.types.str;
      description = "Stable workspace UUID.";
    };
    gui.enable = lib.mkEnableOption "the headless Wayland desktop" // {
      default = true;
    };
  };

  config = lib.mkMerge [
    {
      system.stateVersion = "26.05";
      systemd.network.enable = true;
      networking.useNetworkd = true;
      networking.firewall.allowedTCPPorts = [
        3000
        7070
      ] ++ lib.optional cfg.gui.enable 6080;

      users.groups.workbench.gid = 1000;
      users.users.workbench = {
        uid = 1000;
        isNormalUser = true;
        group = "workbench";
        home = "/home/workbench";
        createHome = true;
        extraGroups = [
          "video"
          "wheel"
        ];
      };
      security.sudo.wheelNeedsPassword = false;

      nix.settings.experimental-features = [
        "nix-command"
        "flakes"
      ];

      environment.systemPackages = with pkgs; [
        bashInteractive
        cargo
        curl
        fd
        gcc
        git
        gnumake
        jq
        nodejs_24
        pi-coding-agent
        python3
        ripgrep
        rustc
      ];

      services.code-server = {
        enable = true;
        user = "workbench";
        group = "workbench";
        host = "0.0.0.0";
        port = 3000;
        auth = "none";
        disableTelemetry = true;
        disableUpdateCheck = true;
        extraPackages = with pkgs; [
          cargo
          gcc
          git
          gnumake
          nodejs_24
          python3
          ripgrep
          rustc
        ];
        extraArguments = [ "/workspace" ];
      };

      systemd.tmpfiles.rules = [
        "d /home/workbench/.pi 0700 workbench workbench -"
        "d /home/workbench/.pi/agent 0700 workbench workbench -"
        "L+ /home/workbench/.pi/agent/models.json - - - - ${piModels}"
      ];
      systemd.services.workbench-prepare-workspace = {
        description = "Prepare the persistent workspace for the unprivileged tools";
        wantedBy = [ "multi-user.target" ];
        before = [
          "code-server.service"
          "workbench-guest-agent.service"
        ];
        unitConfig.RequiresMountsFor = [ "/workspace" ];
        serviceConfig = {
          Type = "oneshot";
          RemainAfterExit = true;
        };
        script = ''
          ${pkgs.coreutils}/bin/install -d -m 0770 -o workbench -g workbench /workspace
          ${pkgs.coreutils}/bin/install -d -m 0700 -o workbench -g workbench /workspace/.pi/sessions
        '';
      };
      systemd.services.code-server = {
        after = [ "workbench-prepare-workspace.service" ];
        requires = [ "workbench-prepare-workspace.service" ];
      };
      systemd.services.workbench-guest-agent = {
        description = "Workbench guest agent with persistent Pi RPC";
        wantedBy = [ "multi-user.target" ];
        after = [
          "local-fs.target"
          "network-online.target"
          "workbench-prepare-workspace.service"
        ];
        requires = [ "workbench-prepare-workspace.service" ];
        wants = [ "network-online.target" ];
        path = with pkgs; [
          bashInteractive
          cargo
          coreutils
          curl
          fd
          gcc
          git
          gnumake
          jq
          nodejs_24
          procps
          python3
          ripgrep
          rustc
          systemd
        ];
        environment = {
          HOME = "/home/workbench";
          PI_EXECUTABLE = "${pkgs.pi-coding-agent}/bin/pi";
          WORKBENCH_GUEST_LISTEN = "0.0.0.0:7070";
          WORKBENCH_WORKSPACE_ROOT = "/workspace";
          WORKBENCH_WORKSPACE_ID = cfg.workspaceId;
        };
        serviceConfig = {
          ExecStart = "${guestAgent}/bin/workbench-guest-agent";
          EnvironmentFile = "%d/model.env";
          LoadCredential = [ "model.env:/run/credentials/workbench/model.env" ];
          Restart = "on-failure";
          RestartSec = 2;
          User = "workbench";
          Group = "workbench";
          WorkingDirectory = "/workspace";
        };
        unitConfig.RequiresMountsFor = [ "/run/credentials/workbench" ];
      };
    }

    (lib.mkIf cfg.gui.enable {
      programs.wayvnc.enable = true;
      environment.systemPackages = with pkgs; [
        blender
        foot
        novnc
        sway
      ];

      systemd.services.workbench-sway = {
        description = "Workbench headless Sway desktop";
        wantedBy = [ "multi-user.target" ];
        environment = {
          HOME = "/home/workbench";
          WLR_BACKENDS = "headless";
          WLR_LIBINPUT_NO_DEVICES = "1";
          WLR_RENDERER = "pixman";
          XDG_RUNTIME_DIR = "/run/workbench-wayland";
          WAYLAND_DISPLAY = "wayland-1";
          XDG_SESSION_TYPE = "wayland";
          GDK_BACKEND = "wayland";
          QT_QPA_PLATFORM = "wayland";
          SDL_VIDEODRIVER = "wayland";
        };
        serviceConfig = {
          ExecStart = "${pkgs.dbus}/bin/dbus-run-session -- ${pkgs.sway}/bin/sway --unsupported-gpu --config ${swayConfig}";
          Restart = "on-failure";
          RestartSec = 2;
          RuntimeDirectory = "workbench-wayland";
          RuntimeDirectoryMode = "0700";
          User = "workbench";
          Group = "workbench";
        };
      };

      systemd.services.workbench-wayvnc = {
        description = "Workbench Wayland VNC server";
        wantedBy = [ "multi-user.target" ];
        after = [ "workbench-sway.service" ];
        requires = [ "workbench-sway.service" ];
        environment = {
          XDG_RUNTIME_DIR = "/run/workbench-wayland";
          WAYLAND_DISPLAY = "wayland-1";
        };
        serviceConfig = {
          ExecStart = startWayvnc;
          Restart = "on-failure";
          RestartSec = 2;
          User = "workbench";
          Group = "workbench";
        };
      };

      systemd.services.workbench-novnc = {
        description = "Workbench noVNC proxy";
        wantedBy = [ "multi-user.target" ];
        after = [ "workbench-wayvnc.service" ];
        requires = [ "workbench-wayvnc.service" ];
        serviceConfig = {
          ExecStart = "${pkgs.novnc}/bin/novnc --listen 6080 --vnc 127.0.0.1:5900";
          Restart = "on-failure";
          RestartSec = 2;
          User = "workbench";
          Group = "workbench";
        };
      };
    })
  ];
}
