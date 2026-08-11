defmodule Workbench.WorkspaceEvent do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id

  schema "workspace_events" do
    field :generation, :integer
    field :kind, :string
    field :payload, :map, default: %{}
    belongs_to :workspace, Workbench.Workspace
    timestamps(type: :utc_datetime_usec, updated_at: false)
  end

  def changeset(event, attrs) do
    event
    |> cast(attrs, [:workspace_id, :generation, :kind, :payload])
    |> validate_required([:workspace_id, :generation, :kind])
  end
end
