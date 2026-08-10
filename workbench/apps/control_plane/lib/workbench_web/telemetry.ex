defmodule WorkbenchWeb.Telemetry do
  use Supervisor

  def start_link(arg), do: Supervisor.start_link(__MODULE__, arg, name: __MODULE__)

  @impl true
  def init(_arg), do: Supervisor.init([], strategy: :one_for_one)
end
