defmodule WorkbenchWeb.ConnCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      @endpoint WorkbenchWeb.Endpoint
      use WorkbenchWeb, :verified_routes
      import Plug.Conn
      import Phoenix.ConnTest
      import Phoenix.LiveViewTest
    end
  end

  setup _tags do
    owner = Ecto.Adapters.SQL.Sandbox.start_owner!(Workbench.Repo, shared: true)
    on_exit(fn -> Ecto.Adapters.SQL.Sandbox.stop_owner(owner) end)
    {:ok, conn: Phoenix.ConnTest.build_conn()}
  end
end
