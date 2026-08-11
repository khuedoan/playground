defmodule Workbench.Repo.Migrations.CreateThreadMessages do
  use Ecto.Migration

  def change do
    create table(:thread_messages, primary_key: false) do
      add :id, :binary_id, primary_key: true

      add :workspace_id, references(:workspaces, type: :binary_id, on_delete: :delete_all),
        null: false

      add :role, :string, null: false
      add :text, :text, null: false
      timestamps(type: :utc_datetime_usec, updated_at: false)
    end

    create index(:thread_messages, [:workspace_id, :inserted_at])
  end
end
