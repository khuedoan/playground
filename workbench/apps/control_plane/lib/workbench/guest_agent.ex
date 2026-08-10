defmodule Workbench.GuestAgent do
  @moduledoc "Client for the Rust guest service and Pi's JSONL RPC bridge."

  @poll_interval 500
  @max_polls 600

  def prompt(%{agent_url: nil}, _message), do: {:error, :agent_url_unavailable}

  def prompt(workspace, message) do
    session = "thread-#{String.replace(workspace.id, "-", "") |> String.slice(0, 24)}"

    with {:ok, baseline} <- state(workspace.agent_url, session),
         {:ok, %{"success" => true}} <-
           rpc(workspace.agent_url, session, %{
             "type" => "prompt",
             "message" => message,
             "id" => Ecto.UUID.generate()
           }),
         :ok <- await_completion(workspace.agent_url, session, baseline["messageCount"] || 0),
         {:ok, %{"data" => reply}} <-
           rpc(workspace.agent_url, session, %{"type" => "get_last_assistant_text"}) do
      extract_last_assistant_text(reply)
    end
  end

  defp state(url, session) do
    with {:ok, %{"data" => data}} <- rpc(url, session, %{"type" => "get_state"}) do
      {:ok, data}
    end
  end

  defp await_completion(url, session, baseline, attempt \\ 0)

  defp await_completion(_url, _session, _baseline, attempt) when attempt >= @max_polls,
    do: {:error, :agent_timeout}

  defp await_completion(url, session, baseline, attempt) do
    Process.sleep(@poll_interval)

    case state(url, session) do
      {:ok, %{"isStreaming" => false, "messageCount" => count}} when count > baseline -> :ok
      {:ok, _state} -> await_completion(url, session, baseline, attempt + 1)
      {:error, reason} -> {:error, reason}
    end
  end

  defp rpc(url, session, command) do
    endpoint = "#{url}/v1/pi/sessions/#{session}/rpc"

    case Req.post(endpoint,
           json: %{request: command},
           receive_timeout: 35_000,
           retry: :transient
         ) do
      {:ok, %{status: status, body: %{"response" => response}}} when status in 200..299 ->
        if response["success"] == false,
          do: {:error, {:pi, response["error"]}},
          else: {:ok, response}

      {:ok, %{status: status, body: body}} ->
        {:error, {:guest_agent, status, body}}

      {:error, reason} ->
        {:error, {:transport, reason}}
    end
  end

  @doc false
  def extract_last_assistant_text(%{"text" => text}) when is_binary(text) do
    case String.trim(text) do
      "" -> {:error, :empty_agent_reply}
      reply -> {:ok, reply}
    end
  end

  def extract_last_assistant_text(_reply), do: {:error, :empty_agent_reply}
end
