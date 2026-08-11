defmodule Workbench.Workspaces do
  import Ecto.Query
  alias Ecto.Multi
  alias Workbench.{Repo, ThreadMessage, Workspace, WorkspaceEvent}
  alias Workbench.Workers.ReconcileWorkspace

  def list_workspaces do
    Repo.all(from workspace in Workspace, order_by: [desc: workspace.inserted_at])
  end

  def get_workspace!(id), do: Repo.get!(Workspace, id)

  def list_messages(workspace_id) do
    Repo.all(
      from message in ThreadMessage,
        where: message.workspace_id == ^workspace_id,
        order_by: [asc: message.inserted_at, asc: message.id]
    )
  end

  def append_message(%Workspace{} = workspace, role, text)
      when role in [:user, :assistant, :error] do
    %ThreadMessage{}
    |> ThreadMessage.changeset(%{workspace_id: workspace.id, role: role, text: text})
    |> Repo.insert()
    |> case do
      {:ok, message} ->
        Phoenix.PubSub.broadcast(
          Workbench.PubSub,
          "workspaces",
          {:message_added, message}
        )

        {:ok, message}

      error ->
        error
    end
  end

  def list_events(workspace_id) do
    Repo.all(
      from event in WorkspaceEvent,
        where: event.workspace_id == ^workspace_id,
        order_by: [asc: event.inserted_at]
    )
  end

  def create_workspace(attrs) do
    Multi.new()
    |> Multi.insert(:workspace, Workspace.create_changeset(%Workspace{}, attrs))
    |> Multi.run(:event, fn repo, %{workspace: workspace} ->
      insert_event(repo, workspace, "workspace.queued", %{})
    end)
    |> Multi.run(:job, fn _repo, %{workspace: workspace} -> enqueue(workspace) end)
    |> Repo.transaction()
    |> case do
      {:ok, %{workspace: workspace}} ->
        broadcast(workspace)
        {:ok, workspace}

      {:error, _step, reason, _changes} ->
        {:error, reason}
    end
  end

  def set_desired(%Workspace{} = workspace, desired_state)
      when desired_state in [:running, :stopped, :deleted] do
    Multi.new()
    |> Multi.update(:workspace, Workspace.desired_state_changeset(workspace, desired_state))
    |> Multi.run(:event, fn repo, %{workspace: updated} ->
      insert_event(repo, updated, "workspace.desired_state", %{desired_state: desired_state})
    end)
    |> Multi.run(:job, fn _repo, %{workspace: updated} -> enqueue(updated) end)
    |> Repo.transaction()
    |> case do
      {:ok, %{workspace: updated}} ->
        broadcast(updated)
        {:ok, updated}

      {:error, _step, reason, _changes} ->
        {:error, reason}
    end
  end

  def mark_reconciling(%Workspace{} = workspace) do
    status =
      case workspace.desired_state do
        :running -> :provisioning
        :stopped -> :stopping
        :deleted -> :deleting
      end

    transition(workspace, %{status: status, failure: nil}, "workspace.reconciling")
  end

  def apply_host_status(%Workspace{} = workspace, host_status, boot_ms) do
    actual = host_status["actual_state"]

    status =
      case actual do
        "running" -> :running
        "stopped" -> :stopped
        "deleted" -> :deleted
        _ -> :failed
      end

    attrs = %{
      status: status,
      ip_address: host_status["ip_address"],
      desktop_url: host_status["desktop_url"],
      code_url: host_status["code_url"],
      agent_url: host_status["agent_url"],
      failure: host_status["error"],
      boot_ms: boot_ms
    }

    transition(workspace, attrs, "workspace.#{status}")
  end

  def mark_failed(%Workspace{} = workspace, reason) do
    transition(
      workspace,
      %{status: :failed, failure: Exception.format_banner(:error, reason)},
      "workspace.failed"
    )
  end

  defp transition(workspace, attrs, kind) do
    Multi.new()
    |> Multi.update(:workspace, Workspace.status_changeset(workspace, attrs))
    |> Multi.run(:event, fn repo, %{workspace: updated} ->
      insert_event(
        repo,
        updated,
        kind,
        Map.new(attrs, fn {key, value} -> {to_string(key), value} end)
      )
    end)
    |> Repo.transaction()
    |> case do
      {:ok, %{workspace: updated}} ->
        broadcast(updated)
        {:ok, updated}

      {:error, _step, reason, _changes} ->
        {:error, reason}
    end
  end

  defp enqueue(workspace) do
    %{workspace_id: workspace.id, generation: workspace.generation}
    |> ReconcileWorkspace.new(
      queue: :provision,
      unique: [period: :infinity, fields: [:worker, :args]]
    )
    |> Oban.insert()
  end

  defp insert_event(repo, workspace, kind, payload) do
    %WorkspaceEvent{}
    |> WorkspaceEvent.changeset(%{
      workspace_id: workspace.id,
      generation: workspace.generation,
      kind: kind,
      payload: payload
    })
    |> repo.insert()
  end

  def subscribe, do: Phoenix.PubSub.subscribe(Workbench.PubSub, "workspaces")

  defp broadcast(workspace) do
    Phoenix.PubSub.broadcast(Workbench.PubSub, "workspaces", {:workspace_updated, workspace})
  end
end
