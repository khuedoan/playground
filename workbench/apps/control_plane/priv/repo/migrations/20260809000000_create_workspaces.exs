defmodule Workbench.Repo.Migrations.CreateWorkspaces do
  use Ecto.Migration

  def change do
    create table(:workspaces, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :title, :string, null: false
      add :status, :string, null: false, default: "queued"
      add :desired_state, :string, null: false, default: "running"
      add :generation, :bigint, null: false, default: 1
      add :command_id, :uuid, null: false
      add :host_id, :string, null: false, default: "local-microvm"
      add :ip_address, :string
      add :desktop_url, :string
      add :code_url, :string
      add :agent_url, :string
      add :failure, :text
      add :boot_ms, :bigint
      timestamps(type: :utc_datetime_usec)
    end

    create unique_index(:workspaces, [:command_id])

    create table(:workspace_events, primary_key: false) do
      add :id, :binary_id, primary_key: true

      add :workspace_id, references(:workspaces, type: :binary_id, on_delete: :delete_all),
        null: false

      add :generation, :bigint, null: false
      add :kind, :string, null: false
      add :payload, :map, null: false, default: %{}
      timestamps(type: :utc_datetime_usec, updated_at: false)
    end

    create index(:workspace_events, [:workspace_id, :inserted_at])
  end
end
