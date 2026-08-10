defmodule Workbench.WorkspacesTest do
  use Workbench.DataCase, async: true
  use Oban.Testing, repo: Workbench.Repo

  alias Workbench.Workers.ReconcileWorkspace
  alias Workbench.Workspaces

  test "creation persists the workspace, audit event, and durable reconciliation job" do
    assert {:ok, workspace} = Workspaces.create_workspace(%{title: "Private dataset"})
    assert workspace.status == :queued
    assert workspace.desired_state == :running

    assert_enqueued(
      worker: ReconcileWorkspace,
      args: %{workspace_id: workspace.id, generation: 1}
    )

    assert [%{kind: "workspace.queued", generation: 1}] = Workspaces.list_events(workspace.id)
  end

  test "each desired-state change receives a new command and generation" do
    {:ok, workspace} = Workspaces.create_workspace(%{title: "Lifecycle"})
    {:ok, stopped} = Workspaces.set_desired(workspace, :stopped)

    assert stopped.generation == workspace.generation + 1
    assert stopped.command_id != workspace.command_id

    assert_enqueued(
      worker: ReconcileWorkspace,
      args: %{workspace_id: workspace.id, generation: stopped.generation}
    )
  end
end
