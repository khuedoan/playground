defmodule Workbench.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      WorkbenchWeb.Telemetry,
      Workbench.Repo,
      {Oban, Application.fetch_env!(:workbench, Oban)},
      {Phoenix.PubSub, name: Workbench.PubSub},
      WorkbenchWeb.Endpoint
    ]

    Supervisor.start_link(children, strategy: :one_for_one, name: Workbench.Supervisor)
  end

  @impl true
  def config_change(changed, _new, removed) do
    WorkbenchWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
