defmodule Workbench.ReconcileWorkspaceTest do
  use Workbench.DataCase, async: false
  use Oban.Testing, repo: Workbench.Repo

  alias Workbench.Workers.ReconcileWorkspace
  alias Workbench.Workspaces

  setup do
    previous = Application.fetch_env!(:workbench, :host_agent_client)
    Application.put_env(:workbench, :host_agent_client, Workbench.FakeHostAgent)
    on_exit(fn -> Application.put_env(:workbench, :host_agent_client, previous) end)
  end

  test "a durable job reconciles desired state and records the audit trail" do
    {:ok, workspace} = Workspaces.create_workspace(%{title: "Reconcile me"})

    assert :ok =
             perform_job(ReconcileWorkspace, %{
               workspace_id: workspace.id,
               generation: workspace.generation
             })

    ready = Workspaces.get_workspace!(workspace.id)
    assert ready.status == :running
    assert ready.ip_address == "172.18.0.8"
    assert ready.desktop_url == "http://127.0.0.1:36080"
    assert ready.boot_ms >= 0

    assert Enum.map(Workspaces.list_events(workspace.id), & &1.kind) == [
             "workspace.queued",
             "workspace.reconciling",
             "workspace.running"
           ]
  end
end
