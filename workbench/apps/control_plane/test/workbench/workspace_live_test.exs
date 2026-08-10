defmodule WorkbenchWeb.WorkspaceLiveTest do
  use WorkbenchWeb.ConnCase, async: false
  use Oban.Testing, repo: Workbench.Repo

  alias Workbench.Workers.ReconcileWorkspace

  test "renders the private workspace dashboard and queues a launch", %{conn: conn} do
    {:ok, view, html} = live(conn, ~p"/")
    assert html =~ "Your private network"
    assert html =~ "No workspaces yet"

    view
    |> form("#new-workspace", workspace: %{title: "Browser-created workspace"})
    |> render_submit()

    assert has_element?(view, "article.workspace-card", "Browser-created workspace")
    assert_enqueued(worker: ReconcileWorkspace)
  end
end
