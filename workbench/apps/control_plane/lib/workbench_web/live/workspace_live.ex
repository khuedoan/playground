defmodule WorkbenchWeb.WorkspaceLive do
  use WorkbenchWeb, :live_view

  alias Workbench.Workspaces

  @impl true
  def mount(_params, _session, socket) do
    if connected?(socket), do: Workspaces.subscribe()

    {:ok,
     socket
     |> assign(:page_title, "Workbench")
     |> assign(:workspace_profile, Application.fetch_env!(:workbench, :workspace_profile))
     |> assign(:pool_size, Application.fetch_env!(:workbench, :pool_size))
     |> assign(:form, to_form(%{"title" => ""}, as: :workspace))
     |> assign(:busy_agents, MapSet.new())
     |> assign(:selected_id, nil)
     |> assign_threads()}
  end

  @impl true
  def handle_event("create", %{"workspace" => %{"title" => title}}, socket) do
    case Workspaces.create_workspace(%{title: String.trim(title)}) do
      {:ok, workspace} ->
        {:noreply,
         socket
         |> assign(:form, to_form(%{"title" => ""}, as: :workspace))
         |> assign_threads(workspace.id)}

      {:error, changeset} ->
        {:noreply, assign(socket, :form, to_form(changeset, as: :workspace))}
    end
  end

  def handle_event("select_thread", %{"id" => id}, socket) do
    {:noreply, assign_threads(socket, id)}
  end

  def handle_event("desired", %{"id" => id, "state" => desired}, socket) do
    workspace = Workspaces.get_workspace!(id)
    {:ok, _updated} = Workspaces.set_desired(workspace, String.to_existing_atom(desired))
    {:noreply, assign_threads(socket, id)}
  end

  def handle_event("prompt", %{"workspace_id" => id, "message" => raw_message}, socket) do
    message = String.trim(raw_message)

    if message == "" do
      {:noreply, socket}
    else
      workspace = Workspaces.get_workspace!(id)
      {:ok, _entry} = Workspaces.append_message(workspace, :user, message)
      owner = self()

      Task.start(fn ->
        send(owner, {:agent_reply, id, Workbench.GuestAgent.prompt(workspace, message)})
      end)

      {:noreply,
       socket
       |> assign(:busy_agents, MapSet.put(socket.assigns.busy_agents, id))
       |> assign_threads(id)}
    end
  end

  @impl true
  def handle_info({:workspace_updated, _workspace}, socket) do
    {:noreply, assign_threads(socket)}
  end

  def handle_info({:message_added, message}, socket) do
    if message.workspace_id == socket.assigns.selected_id do
      {:noreply, assign(socket, :messages, Workspaces.list_messages(message.workspace_id))}
    else
      {:noreply, socket}
    end
  end

  def handle_info({:agent_reply, id, result}, socket) do
    workspace = Workspaces.get_workspace!(id)

    {role, text} =
      case result do
        {:ok, text} -> {:assistant, text}
        {:error, reason} -> {:error, "Agent error: #{inspect(reason)}"}
      end

    {:ok, _entry} = Workspaces.append_message(workspace, role, text)

    {:noreply,
     socket
     |> assign(:busy_agents, MapSet.delete(socket.assigns.busy_agents, id))
     |> assign_threads()}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="workbench-shell">
      <aside class="thread-sidebar">
        <header class="sidebar-brand">
          <div class="brand-mark">›_</div>
          <div>
            <strong>Workbench</strong>
            <span>MicroVM agents</span>
          </div>
        </header>

        <.form for={@form} id="new-workspace" phx-submit="create" class="new-thread-form">
          <.input field={@form[:title]} placeholder="Name a new thread" autocomplete="off" />
          <button type="submit" aria-label="Start new thread">+</button>
        </.form>

        <div class="thread-section-label">
          <span>Threads</span>
          <small>{active_thread_count(@workspaces)} / {@pool_size}</small>
        </div>

        <nav class="thread-list" aria-label="Agent threads">
          <button
            :for={workspace <- @workspaces}
            type="button"
            id={"thread-#{workspace.id}"}
            class={["thread-item", @selected_id == workspace.id && "is-active"]}
            phx-click="select_thread"
            phx-value-id={workspace.id}
          >
            <span class={["thread-status", "thread-status-#{workspace.status}"]}></span>
            <span class="thread-copy">
              <strong>{workspace.title}</strong>
              <small>{thread_summary(workspace, @busy_agents)}</small>
            </span>
            <time>{boot_time(workspace)}</time>
          </button>
        </nav>

        <div :if={@workspaces == []} class="sidebar-empty">
          <span>⌘</span>
          <p>Create a thread to lease a ready, isolated Linux workspace.</p>
        </div>

        <footer class="sidebar-footer">
          <span class="pool-dot"></span>
          <div>
            <strong>Warm pool online</strong>
            <small>MicroVMs ready to lease</small>
          </div>
        </footer>
      </aside>

      <main class="thread-main">
        <section :if={is_nil(@selected_workspace)} class="welcome-panel">
          <div class="welcome-glyph">›_</div>
          <h1>Start a thread</h1>
          <p>Each agent gets an isolated Wayland workspace with code-server, shell access, and durable storage.</p>
        </section>

        <%= if @selected_workspace do %>
          <header class="thread-header">
            <div>
              <p>Agent thread</p>
              <h1>{@selected_workspace.title}</h1>
            </div>
            <span class={["status-pill", "status-#{@selected_workspace.status}"]}>
              <i></i>{human_status(@selected_workspace.status)}
            </span>
          </header>

          <section class="conversation-feed" id={"conversation-#{@selected_workspace.id}"}>
            <div :if={@messages == []} class="conversation-starter">
              <div class="agent-avatar">›_</div>
              <h2>What should I work on?</h2>
              <p>
                Ask the agent to inspect files, run commands, edit code, or use the graphical workspace.
                This thread and its files persist independently.
              </p>
              <div class="suggestion-row">
                <span>Inspect this workspace</span>
                <span>Run the test suite</span>
                <span>Open Blender</span>
              </div>
            </div>

            <article :for={entry <- @messages} class={["message", "message-#{entry.role}"]}>
              <div class="message-avatar">{if entry.role == :user, do: "Y", else: "›_"}</div>
              <div>
                <strong>{if entry.role == :user, do: "You", else: "Workbench"}</strong>
                <p>{entry.text}</p>
              </div>
            </article>

            <div
              :if={MapSet.member?(@busy_agents, @selected_workspace.id)}
              class="agent-progress"
            >
              <span></span><span></span><span></span>
              Agent is working in the MicroVM
            </div>
          </section>

          <footer class="composer-wrap">
            <form
              :if={@selected_workspace.status == :running}
              id={"prompt-#{@selected_workspace.id}"}
              phx-submit="prompt"
              class="prompt-form"
            >
              <input type="hidden" name="workspace_id" value={@selected_workspace.id} />
              <textarea
                name="message"
                placeholder="Ask Workbench to make a change…"
                disabled={MapSet.member?(@busy_agents, @selected_workspace.id)}
                required
              ></textarea>
              <div class="composer-meta">
                <span>Local tools · isolated network</span>
                <button
                  type="submit"
                  aria-label="Send message"
                  disabled={MapSet.member?(@busy_agents, @selected_workspace.id)}
                >↑</button>
              </div>
            </form>

            <div :if={@selected_workspace.status in [:queued, :provisioning]} class="lifecycle-note">
              <span class="spinner"></span>
              Leasing a prewarmed MicroVM…
            </div>
            <div :if={@selected_workspace.status == :failed} class="lifecycle-note is-error">
              {@selected_workspace.failure}
            </div>
            <button
              :if={@selected_workspace.status in [:stopped, :failed]}
              class="start-thread"
              phx-click="desired"
              phx-value-id={@selected_workspace.id}
              phx-value-state="running"
            >Start thread</button>
          </footer>
        <% end %>
      </main>

      <aside class="workspace-inspector">
        <%= if @selected_workspace do %>
          <header>
            <div>
              <p>Workspace</p>
              <h2>Live environment</h2>
            </div>
            <span class="secure-label"><i></i> isolated</span>
          </header>

          <div :if={@selected_workspace.status == :running} class="desktop-frame">
            <div class="desktop-bar">
              <span></span><span></span><span></span>
              <strong>Wayland</strong>
              <a href={@selected_workspace.desktop_url} target="_blank">Open ↗</a>
            </div>
            <iframe
              src={@selected_workspace.desktop_url}
              title={"#{@selected_workspace.title} desktop"}
            ></iframe>
          </div>

          <div :if={@selected_workspace.status != :running} class="desktop-placeholder">
            <div>›_</div>
            <p>{human_status(@selected_workspace.status)}</p>
          </div>

          <dl class="workspace-facts">
            <div>
              <dt>MicroVM</dt>
              <dd>{String.slice(@selected_workspace.id, 0, 8)}</dd>
            </div>
            <div>
              <dt>Address</dt>
              <dd>{@selected_workspace.ip_address || "waiting"}</dd>
            </div>
            <div>
              <dt>Resources</dt>
              <dd>{@workspace_profile.vcpus} CPU · {memory_gib(@workspace_profile)} GB</dd>
            </div>
            <div>
              <dt>Ready</dt>
              <dd>{boot_time(@selected_workspace) || "—"}</dd>
            </div>
          </dl>

          <div :if={@selected_workspace.status == :running} class="inspector-actions">
            <a href={@selected_workspace.code_url} target="_blank" class="open-code">Open code-server ↗</a>
            <button
              phx-click="desired"
              phx-value-id={@selected_workspace.id}
              phx-value-state="stopped"
            >Stop</button>
          </div>
          <button
            :if={@selected_workspace.status != :deleted}
            class="delete-thread"
            phx-click="desired"
            phx-value-id={@selected_workspace.id}
            phx-value-state="deleted"
          >Delete thread and reset workspace</button>
        <% else %>
          <div class="inspector-empty">
            <span>□</span>
            <p>Select a thread to inspect its desktop and runtime.</p>
          </div>
        <% end %>
      </aside>
    </div>
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

  defp assign_threads(socket, preferred_id \\ nil) do
    workspaces = Workspaces.list_workspaces()
    requested_id = preferred_id || socket.assigns.selected_id

    selected =
      Enum.find(workspaces, &(&1.id == requested_id)) ||
        Enum.find(workspaces, &(&1.status != :deleted)) ||
        List.first(workspaces)

    socket
    |> assign(:workspaces, workspaces)
    |> assign(:selected_id, selected && selected.id)
    |> assign(:selected_workspace, selected)
    |> assign(:messages, if(selected, do: Workspaces.list_messages(selected.id), else: []))
  end

  defp human_status(status), do: status |> Atom.to_string() |> String.replace("_", " ")

  defp thread_summary(workspace, busy_agents) do
    cond do
      MapSet.member?(busy_agents, workspace.id) -> "working"
      workspace.status == :running -> "ready · #{workspace.ip_address || "networking"}"
      true -> human_status(workspace.status)
    end
  end

  defp boot_time(%{boot_ms: nil}), do: nil
  defp boot_time(%{boot_ms: ms}) when ms < 1_000, do: "#{ms} ms"
  defp boot_time(%{boot_ms: ms}), do: "#{Float.round(ms / 1_000, 1)} s"
  defp memory_gib(profile), do: Float.round(profile.memory_mib / 1_024, 1)
  defp active_thread_count(workspaces), do: Enum.count(workspaces, &(&1.status != :deleted))
end
