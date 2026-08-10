defmodule Workbench.HostAgent do
  @moduledoc "Contract implemented by host-agent transports."

  @callback ensure(map()) :: {:ok, map()} | {:error, term()}

  def ensure(payload) do
    Application.fetch_env!(:workbench, :host_agent_client).ensure(payload)
  end
end

defmodule Workbench.HostAgent.Http do
  @behaviour Workbench.HostAgent

  @impl true
  def ensure(%{"workspace_id" => workspace_id} = payload) do
    base = Application.fetch_env!(:workbench, :host_agent_url)

    case Req.put("#{base}/v1/workspaces/#{workspace_id}", json: payload, receive_timeout: 240_000) do
      {:ok, %{status: status, body: body}} when status in 200..299 -> {:ok, body}
      {:ok, %{status: status, body: body}} -> {:error, {:host_agent, status, body}}
      {:error, reason} -> {:error, {:transport, reason}}
    end
  end
end
