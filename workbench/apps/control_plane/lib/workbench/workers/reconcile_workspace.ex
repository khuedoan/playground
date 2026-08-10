defmodule Workbench.Workers.ReconcileWorkspace do
  use Oban.Worker, queue: :provision, max_attempts: 8

  alias Workbench.{HostAgent, Workspaces}

  @impl Oban.Worker
  def perform(%Oban.Job{args: %{"workspace_id" => id, "generation" => generation}}) do
    workspace = Workspaces.get_workspace!(id)

    if workspace.generation != generation do
      :discard
    else
      started_at = System.monotonic_time(:millisecond)

      with {:ok, reconciling} <- Workspaces.mark_reconciling(workspace),
           {:ok, status} <- HostAgent.ensure(payload(reconciling)),
           boot_ms = System.monotonic_time(:millisecond) - started_at,
           {:ok, _workspace} <- Workspaces.apply_host_status(reconciling, status, boot_ms) do
        :ok
      else
        {:error, reason} ->
          Workspaces.mark_failed(workspace, reason)
          {:error, reason}
      end
    end
  end

  defp payload(workspace) do
    %{
      "command_id" => workspace.command_id,
      "workspace_id" => workspace.id,
      "generation" => workspace.generation,
      "desired_state" => Atom.to_string(workspace.desired_state),
      "profile" => %{
        "vcpus" => 4,
        "memory_mib" => 8192,
        "disk_gib" => 40,
        "gui" => true
      }
    }
  end
end
