defmodule WorkbenchWeb.WorkspaceLiveTest do
  use WorkbenchWeb.ConnCase, async: false
  use Oban.Testing, repo: Workbench.Repo

  alias Workbench.Workers.ReconcileWorkspace

  test "renders the agent workbench and queues multiple independent threads", %{conn: conn} do
    {:ok, view, html} = live(conn, ~p"/")
    assert html =~ "Start a thread"
    assert html =~ "Warm pool online"

    view
    |> form("#new-workspace", workspace: %{title: "Browser-created workspace"})
    |> render_submit()

    view
    |> form("#new-workspace", workspace: %{title: "Parallel workspace"})
    |> render_submit()

    assert has_element?(view, "#thread-" <> first_id(), "Browser-created workspace")
    assert has_element?(view, ".thread-item", "Parallel workspace")
    assert length(all_enqueued(worker: ReconcileWorkspace)) == 2
  end

  defp first_id do
    Workbench.Workspaces.list_workspaces()
    |> Enum.find(&(&1.title == "Browser-created workspace"))
    |> Map.fetch!(:id)
  end
end
