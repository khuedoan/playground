defmodule Workbench.ThreadMessage do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id

  schema "thread_messages" do
    field :role, Ecto.Enum, values: [:user, :assistant, :error]
    field :text, :string
    belongs_to :workspace, Workbench.Workspace
    timestamps(type: :utc_datetime_usec, updated_at: false)
  end

  def changeset(message, attrs) do
    message
    |> cast(attrs, [:workspace_id, :role, :text])
    |> validate_required([:workspace_id, :role, :text])
    |> validate_length(:text, min: 1, max: 100_000)
  end
end
