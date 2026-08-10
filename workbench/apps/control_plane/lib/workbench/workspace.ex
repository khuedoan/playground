defmodule Workbench.Workspace do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id

  schema "workspaces" do
    field :title, :string

    field :status, Ecto.Enum,
      values: [
        :queued,
        :provisioning,
        :running,
        :stopping,
        :stopped,
        :deleting,
        :deleted,
        :failed
      ],
      default: :queued

    field :desired_state, Ecto.Enum, values: [:running, :stopped, :deleted], default: :running
    field :generation, :integer, default: 1
    field :command_id, Ecto.UUID
    field :host_id, :string, default: "local-microvm"
    field :ip_address, :string
    field :desktop_url, :string
    field :code_url, :string
    field :agent_url, :string
    field :failure, :string
    field :boot_ms, :integer

    timestamps(type: :utc_datetime_usec)
  end

  def create_changeset(workspace, attrs) do
    workspace
    |> cast(attrs, [:title, :host_id])
    |> put_change(:command_id, Ecto.UUID.generate())
    |> validate_required([:title, :command_id])
    |> validate_length(:title, min: 1, max: 120)
  end

  def desired_state_changeset(workspace, desired_state) do
    workspace
    |> change(%{
      desired_state: desired_state,
      generation: workspace.generation + 1,
      command_id: Ecto.UUID.generate(),
      failure: nil
    })
  end

  def status_changeset(workspace, attrs) do
    cast(workspace, attrs, [
      :status,
      :ip_address,
      :desktop_url,
      :code_url,
      :agent_url,
      :failure,
      :boot_ms
    ])
  end
end
