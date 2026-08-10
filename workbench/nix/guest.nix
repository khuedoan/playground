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
  modelProvider = if cfg.mockLlm.enable then "e2e-mock" else "local-llama";
  modelId = if cfg.mockLlm.enable then "workbench-e2e-mock" else "local-coder";
  modelName = if cfg.mockLlm.enable then "Workbench E2E Mock" else "Local Qwen2.5 Coder 0.5B";
  modelService = if cfg.mockLlm.enable then "workbench-mock-llm.service" else "llama-cpp.service";
  localCoderModel = pkgs.fetchurl {
    name = "qwen2.5-coder-0.5b-instruct-q4_0.gguf";
    url = "https://huggingface.co/Qwen/Qwen2.5-Coder-0.5B-Instruct-GGUF/resolve/56c037aa51c4d689c272c21ea2a8b9c13341e8b2/qwen2.5-coder-0.5b-instruct-q4_0.gguf";
    hash = "sha256-lzkFXgRtYqk35beHkBIgnvQOvqihVpqWAo3kkfPwkdU=";
  };
  piModels = pkgs.writeText "workbench-pi-models.json" (
    builtins.toJSON {
      providers.${modelProvider} = {
        baseUrl = "http://127.0.0.1:8080/v1";
        api = "openai-completions";
        apiKey = if cfg.mockLlm.enable then "mock" else "none";
        compat = {
          supportsDeveloperRole = false;
          supportsReasoningEffort = false;
        };
        models = [
          {
            id = modelId;
            name = modelName;
            reasoning = false;
            input = [ "text" ];
            contextWindow = 8192;
            maxTokens = 512;
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
    mockLlm.enable = lib.mkEnableOption "the deterministic mock LLM used by end-to-end tests";
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
      services.llama-cpp = lib.mkIf (!cfg.mockLlm.enable) {
        enable = true;
        settings = {
          model = localCoderModel;
          alias = "local-coder";
          host = "127.0.0.1";
          port = 8080;
          ctx-size = 8192;
          n-predict = 512;
          threads = 2;
          parallel = 1;
          temp = 0.1;
          jinja = true;
        };
      };
      systemd.services.llama-cpp = lib.mkIf (!cfg.mockLlm.enable) {
        unitConfig.OnFailure = [ "workbench-guest-agent-diagnostics.service" ];
        serviceConfig.RestartSec = lib.mkForce 2;
      };
      systemd.services.workbench-mock-llm = lib.mkIf cfg.mockLlm.enable {
        description = "Deterministic OpenAI-compatible LLM for end-to-end tests";
        serviceConfig = {
          ExecStart = "${pkgs.python3}/bin/python3 ${../scripts/mock_llm.py}";
          Restart = "on-failure";
          RestartSec = 1;
          User = "workbench";
          Group = "workbench";
        };
        unitConfig.OnFailure = [ "workbench-guest-agent-diagnostics.service" ];
      };
      systemd.services.workbench-model-ready = {
        description = "Wait for the configured LLM API";
        after = [ modelService ];
        requires = [ modelService ];
        before = [ "workbench-guest-agent.service" ];
        unitConfig.OnFailure = [ "workbench-guest-agent-diagnostics.service" ];
        serviceConfig = {
          Type = "oneshot";
          RemainAfterExit = true;
        };
        script = ''
          for attempt in $(${pkgs.coreutils}/bin/seq 1 240); do
            if ${pkgs.curl}/bin/curl --fail --silent http://127.0.0.1:8080/health >/dev/null; then
              exit 0
            fi
            ${pkgs.coreutils}/bin/sleep 0.5
          done
          echo "LLM API did not become ready" >&2
          exit 1
        '';
      };
      systemd.services.workbench-guest-agent = {
        description = "Workbench guest agent with persistent Pi RPC";
        wantedBy = [ "multi-user.target" ];
        after = [
          "local-fs.target"
          "network-online.target"
          "workbench-model-ready.service"
          "workbench-prepare-workspace.service"
        ];
        requires = [
          "workbench-model-ready.service"
          "workbench-prepare-workspace.service"
        ];
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
          PI_API_KEY = if cfg.mockLlm.enable then "mock" else "none";
          PI_MODEL = modelId;
          PI_PROVIDER = modelProvider;
          WORKBENCH_GUEST_LISTEN = "0.0.0.0:7070";
          WORKBENCH_WORKSPACE_ROOT = "/workspace";
          WORKBENCH_WORKSPACE_ID = cfg.workspaceId;
        };
        serviceConfig = {
          ExecStart = "${guestAgent}/bin/workbench-guest-agent";
          Restart = "on-failure";
          RestartSec = 2;
          User = "workbench";
          Group = "workbench";
          WorkingDirectory = "/workspace";
        };
        unitConfig.OnFailure = [ "workbench-guest-agent-diagnostics.service" ];
      };
      systemd.services.workbench-guest-agent-diagnostics = {
        description = "Print a failed guest-agent unit status to the serial console";
        serviceConfig = {
          Type = "oneshot";
          StandardOutput = "journal+console";
          StandardError = "journal+console";
        };
        script = ''
          ${pkgs.curl}/bin/curl --silent --show-error http://127.0.0.1:8080/health || true
          ${pkgs.systemd}/bin/systemctl --no-pager --full status ${modelService} workbench-model-ready.service || true
          ${pkgs.systemd}/bin/journalctl --no-pager -u ${modelService} -u workbench-model-ready.service -n 80 || true
          ${pkgs.systemd}/bin/systemctl --no-pager --full status workbench-guest-agent.service || true
          ${pkgs.systemd}/bin/journalctl --no-pager -u workbench-guest-agent.service -n 40 || true
        '';
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
        path = [
          pkgs.bash
          pkgs.dbus
        ];
        environment = {
          HOME = "/home/workbench";
          WLR_BACKENDS = "headless";
          WLR_HEADLESS_OUTPUTS = "1";
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
        unitConfig.OnFailure = [ "workbench-gui-diagnostics.service" ];
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
        unitConfig.OnFailure = [ "workbench-gui-diagnostics.service" ];
      };

      systemd.services.workbench-novnc = {
        description = "Workbench noVNC proxy";
        wantedBy = [ "multi-user.target" ];
        after = [ "workbench-wayvnc.service" ];
        requires = [ "workbench-wayvnc.service" ];
        path = [ pkgs.procps ];
        serviceConfig = {
          ExecStart = "${pkgs.novnc}/bin/novnc --listen 0.0.0.0:6080 --vnc 127.0.0.1:5900";
          Restart = "on-failure";
          RestartSec = 2;
          User = "workbench";
          Group = "workbench";
        };
        unitConfig.OnFailure = [ "workbench-gui-diagnostics.service" ];
      };

      systemd.services.workbench-gui-diagnostics = {
        description = "Print failed GUI unit diagnostics to the serial console";
        serviceConfig = {
          Type = "oneshot";
          StandardOutput = "journal+console";
          StandardError = "journal+console";
        };
        script = ''
          ${pkgs.systemd}/bin/systemctl --no-pager --full status workbench-sway.service workbench-wayvnc.service workbench-novnc.service || true
          ${pkgs.systemd}/bin/journalctl --no-pager -u workbench-sway.service -u workbench-wayvnc.service -u workbench-novnc.service -n 120 || true
          ${pkgs.iproute2}/bin/ss -ltnp || true
        '';
      };
    })
  ];
}
