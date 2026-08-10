defmodule Workbench.DataCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      alias Workbench.Repo
      import Ecto
      import Ecto.Changeset
      import Ecto.Query
      import Workbench.DataCase
    end
  end

  setup tags do
    owner = Ecto.Adapters.SQL.Sandbox.start_owner!(Workbench.Repo, shared: not tags[:async])
    on_exit(fn -> Ecto.Adapters.SQL.Sandbox.stop_owner(owner) end)
    :ok
  end
end
