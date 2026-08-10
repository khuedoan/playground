defmodule Workbench.FakeHostAgent do
  @behaviour Workbench.HostAgent

  @impl true
  def ensure(payload) do
    {:ok,
     %{
       "workspace_id" => payload["workspace_id"],
       "generation" => payload["generation"],
       "command_id" => payload["command_id"],
       "desired_state" => payload["desired_state"],
       "actual_state" => payload["desired_state"],
       "ip_address" => "172.18.0.8",
       "desktop_url" => "http://127.0.0.1:36080",
       "code_url" => "http://127.0.0.1:33000",
       "agent_url" => "http://172.18.0.8:7070",
       "error" => nil
     }}
  end
end
