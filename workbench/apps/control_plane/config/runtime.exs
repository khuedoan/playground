import Config

if config_env() == :prod do
  database_url =
    System.get_env("DATABASE_URL") ||
      raise "DATABASE_URL is required, for example ecto://USER:PASS@HOST/DATABASE"

  secret_key_base =
    System.get_env("SECRET_KEY_BASE") ||
      raise "SECRET_KEY_BASE is required (generate one with mix phx.gen.secret)"

  config :workbench, Workbench.Repo,
    url: database_url,
    pool_size: String.to_integer(System.get_env("POOL_SIZE", "10"))

  config :workbench, WorkbenchWeb.Endpoint,
    server: true,
    http: [ip: {0, 0, 0, 0}, port: String.to_integer(System.get_env("PORT", "4000"))],
    secret_key_base: secret_key_base
end

config :workbench, :host_agent_url, System.get_env("HOST_AGENT_URL", "http://127.0.0.1:9090")

config :workbench,
       :host_agent_timeout_ms,
       String.to_integer(System.get_env("HOST_AGENT_TIMEOUT_MS", "900000"))

config :workbench, :workspace_profile, %{
  vcpus: String.to_integer(System.get_env("WORKBENCH_VCPUS", "4")),
  memory_mib: String.to_integer(System.get_env("WORKBENCH_MEMORY_MIB", "8192")),
  disk_gib: String.to_integer(System.get_env("WORKBENCH_DISK_GIB", "40")),
  gui: true
}
