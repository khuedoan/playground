import Config

config :workbench, Workbench.Repo,
  username: System.get_env("POSTGRES_USER", "postgres"),
  password: System.get_env("POSTGRES_PASSWORD", "postgres"),
  hostname: System.get_env("POSTGRES_HOST", "localhost"),
  port: String.to_integer(System.get_env("POSTGRES_PORT", "5432")),
  database: System.get_env("POSTGRES_DB", "workbench_test"),
  pool: Ecto.Adapters.SQL.Sandbox,
  pool_size: 10

config :workbench, Oban, testing: :manual, queues: false, plugins: false

config :workbench, WorkbenchWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4002],
  secret_key_base:
    "test-secret-key-base-test-secret-key-base-test-secret-key-base-test-secret-key-base-1234",
  server: false

config :logger, level: :warning
config :phoenix, :plug_init_mode, :runtime
