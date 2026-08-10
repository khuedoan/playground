import Config

config :workbench,
  ecto_repos: [Workbench.Repo],
  host_agent_client: Workbench.HostAgent.Http,
  host_agent_timeout_ms: 900_000,
  workspace_profile: %{vcpus: 4, memory_mib: 8192, disk_gib: 40, gui: true}

config :workbench, Oban,
  repo: Workbench.Repo,
  plugins: [Oban.Plugins.Pruner],
  queues: [provision: 10]

config :workbench, WorkbenchWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [html: WorkbenchWeb.ErrorHTML, json: WorkbenchWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: Workbench.PubSub,
  live_view: [signing_salt: "workbench-live"]

config :phoenix, :json_library, Jason

import_config "#{config_env()}.exs"
