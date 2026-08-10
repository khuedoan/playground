defmodule WorkbenchWeb.WorkspaceLive do
  use WorkbenchWeb, :live_view

  alias Workbench.Workspaces

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: Workspaces.subscribe()

    {:ok,
     socket
     |> assign(:page_title, "Private agent workspaces")
     |> assign(:form, to_form(%{"title" => ""}, as: :workspace))
     |> assign(:conversations, %{})
     |> assign(:busy_agents, MapSet.new())
     |> assign(:workspaces, Workspaces.list_workspaces())}
  end

  @impl true
  def handle_event("create", %{"workspace" => %{"title" => title}}, socket) do
    case Workspaces.create_workspace(%{title: String.trim(title)}) do
      {:ok, _workspace} ->
        {:noreply,
         socket
         |> assign(:form, to_form(%{"title" => ""}, as: :workspace))
         |> assign(:workspaces, Workspaces.list_workspaces())}

      {:error, changeset} ->
        {:noreply, assign(socket, :form, to_form(changeset, as: :workspace))}
    end
  end

  def handle_event("desired", %{"id" => id, "state" => desired}, socket) do
    workspace = Workspaces.get_workspace!(id)
    {:ok, _updated} = Workspaces.set_desired(workspace, String.to_existing_atom(desired))
    {:noreply, assign(socket, :workspaces, Workspaces.list_workspaces())}
  end

  def handle_event("prompt", %{"workspace_id" => id, "message" => raw_message}, socket) do
    message = String.trim(raw_message)

    if message == "" do
      {:noreply, socket}
    else
      workspace = Workspaces.get_workspace!(id)
      owner = self()

      Task.start(fn ->
        send(owner, {:agent_reply, id, Workbench.GuestAgent.prompt(workspace, message)})
      end)

      entry = %{role: :user, text: message}
      conversations = Map.update(socket.assigns.conversations, id, [entry], &(&1 ++ [entry]))

      {:noreply,
       socket
       |> assign(:conversations, conversations)
       |> assign(:busy_agents, MapSet.put(socket.assigns.busy_agents, id))}
    end
  end

  @impl true
  def handle_info({:workspace_updated, _workspace}, socket) do
    {:noreply, assign(socket, :workspaces, Workspaces.list_workspaces())}
  end

  def handle_info({:agent_reply, id, result}, socket) do
    entry =
      case result do
        {:ok, text} -> %{role: :assistant, text: text}
        {:error, reason} -> %{role: :error, text: "Agent error: #{inspect(reason)}"}
      end

    conversations = Map.update(socket.assigns.conversations, id, [entry], &(&1 ++ [entry]))

    {:noreply,
     socket
     |> assign(:conversations, conversations)
     |> assign(:busy_agents, MapSet.delete(socket.assigns.busy_agents, id))}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <section class="hero">
      <div>
        <p class="eyebrow">Isolated Linux environments</p>
        <h1>Your private network,<br /><span>one agent per workspace.</span></h1>
        <p class="hero-copy">
          Phoenix keeps the workflow durable. Rust reconciles MicroVMs. Pi works inside its own NixOS guest.
        </p>
      </div>

      <.form for={@form} id="new-workspace" phx-submit="create" class="create-card">
        <label for={@form[:title].id}>Start a workspace</label>
        <div class="create-row">
          <.input
            field={@form[:title]}
            placeholder="e.g. Inspect customer dataset"
            autocomplete="off"
          />
          <button type="submit">Launch</button>
        </div>
        <p>4 vCPU · 8 GB · KVM isolation · Wayland desktop</p>
      </.form>
    </section>

    <section class="workspace-section">
      <div class="section-heading">
        <h2>Workspaces</h2>
        <span>{length(@workspaces)} total</span>
      </div>

      <div :if={@workspaces == []} class="empty-state">
        <div class="empty-glyph">›_</div>
        <h3>No workspaces yet</h3>
        <p>Create one above. The lifecycle will continue even if you close this tab.</p>
      </div>

      <article :for={workspace <- @workspaces} id={"workspace-#{workspace.id}"} class="workspace-card">
        <div class="workspace-summary">
          <div class="workspace-icon">{String.first(workspace.title)}</div>
          <div>
            <h3>{workspace.title}</h3>
            <p>{String.slice(workspace.id, 0, 8)} · generation {workspace.generation}</p>
          </div>
          <span class={"status status-#{workspace.status}"}>
            <i></i>{human_status(workspace.status)}
          </span>
          <span class="boot-time">{boot_time(workspace)}</span>
        </div>

        <div :if={workspace.status == :running} class="workspace-body">
          <div class="desktop-frame">
            <div class="desktop-bar">
              <span></span><span></span><span></span>
              <strong>Wayland desktop</strong>
              <a href={workspace.desktop_url} target="_blank">Open ↗</a>
            </div>
            <iframe src={workspace.desktop_url} title={"#{workspace.title} desktop"}></iframe>
          </div>
          <aside class="workspace-actions">
            <div>
              <span>MicroVM IP</span>
              <strong>{workspace.ip_address || "discovering"}</strong>
            </div>
            <a href={workspace.code_url} target="_blank" class="primary-action">Open code-server ↗</a>
            <button phx-click="desired" phx-value-id={workspace.id} phx-value-state="stopped">
              Stop workspace
            </button>
            <button
              class="danger"
              phx-click="desired"
              phx-value-id={workspace.id}
              phx-value-state="deleted"
            >
              Delete
            </button>
          </aside>
        </div>

        <section :if={workspace.status == :running} class="agent-panel">
          <header>
            <div class="pi-mark">π</div>
            <div>
              <h4>Pi coding agent</h4>
              <p>Runs inside this workspace with access to its Linux environment.</p>
            </div>
            <span :if={MapSet.member?(@busy_agents, workspace.id)} class="agent-working">working…</span>
          </header>

          <div class="conversation" id={"conversation-#{workspace.id}"}>
            <p :if={Map.get(@conversations, workspace.id, []) == []} class="conversation-empty">
              Ask Pi to inspect files, run tests, or work in Blender.
            </p>
            <div
              :for={entry <- Map.get(@conversations, workspace.id, [])}
              class={"message message-#{entry.role}"}
            >
              <strong>{if entry.role == :user, do: "You", else: "Pi"}</strong>
              <p>{entry.text}</p>
            </div>
          </div>

          <form phx-submit="prompt" class="prompt-form">
            <input type="hidden" name="workspace_id" value={workspace.id} />
            <textarea
              name="message"
              placeholder="Ask Pi to do something in this workspace…"
              disabled={MapSet.member?(@busy_agents, workspace.id)}
              required
            ></textarea>
            <button type="submit" disabled={MapSet.member?(@busy_agents, workspace.id)}>Send</button>
          </form>
        </section>

        <div :if={workspace.status in [:stopped, :failed]} class="inline-actions">
          <p :if={workspace.failure}>{workspace.failure}</p>
          <button phx-click="desired" phx-value-id={workspace.id} phx-value-state="running">Start workspace</button>
          <button
            class="danger"
            phx-click="desired"
            phx-value-id={workspace.id}
            phx-value-state="deleted"
          >Delete</button>
        </div>
      </article>
    </section>
    """
  end

  attr :field, Phoenix.HTML.FormField, required: true
  attr :placeholder, :string, default: nil
  attr :autocomplete, :string, default: nil

  defp input(assigns) do
    ~H"""
    <input
      type="text"
      id={@field.id}
      name={@field.name}
      value={@field.value}
      placeholder={@placeholder}
      autocomplete={@autocomplete}
      required
    />
    """
  end

  defp human_status(status), do: status |> Atom.to_string() |> String.replace("_", " ")
  defp boot_time(%{boot_ms: nil}), do: ""
  defp boot_time(%{boot_ms: ms}), do: "ready in #{Float.round(ms / 1000, 1)}s"
end
