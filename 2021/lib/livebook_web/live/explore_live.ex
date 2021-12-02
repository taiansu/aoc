defmodule LivebookWeb.ExploreLive do
  use LivebookWeb, :live_view

  import LivebookWeb.SessionHelpers
  import LivebookWeb.UserHelpers

  alias LivebookWeb.{SidebarHelpers, ExploreHelpers, PageHelpers}
  alias Livebook.Notebook.Explore

  @impl true
  def mount(_params, _session, socket) do
    [lead_notebook_info | notebook_infos] = Explore.visible_notebook_infos()

    {:ok,
     assign(socket,
       lead_notebook_info: lead_notebook_info,
       notebook_infos: notebook_infos
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-grow h-full">
      <SidebarHelpers.sidebar>
        <SidebarHelpers.logo_item socket={@socket} />
        <SidebarHelpers.break_item />
        <SidebarHelpers.link_item
          icon="settings-3-fill"
          label="Settings"
          path={Routes.settings_path(@socket, :page)}
          active={false} />
        <SidebarHelpers.user_item current_user={@current_user} path={Routes.explore_path(@socket, :user)} />
      </SidebarHelpers.sidebar>
      <div class="flex-grow px-6 py-8 overflow-y-auto">
        <div class="max-w-screen-md w-full mx-auto px-4 pb-8 space-y-8">
          <div>
            <PageHelpers.title text="Explore" socket={@socket} />
            <p class="mt-4 text-gray-700">
              Check out a number of examples showcasing various parts of the Elixir ecosystem.
              Click on any notebook you like and start playing around with it!
            </p>
          </div>
          <div class="p-8 bg-gray-900 rounded-2xl flex space-x-4 shadow-xl">
            <div class="self-end max-w-sm">
              <h3 class="text-xl text-gray-50 font-semibold">
                <%= @lead_notebook_info.title %>
              </h3>
              <p class="mt-2 text-sm text-gray-300">
                <%= @lead_notebook_info.details.description %>
              </p>
              <div class="mt-4">
                <%= live_patch "Let's go",
                      to: Routes.explore_path(@socket, :notebook, @lead_notebook_info.slug),
                      class: "button button-blue" %>
              </div>
            </div>
            <div class="flex-grow hidden md:flex flex items-center justify-center">
              <img src={@lead_notebook_info.details.cover_url} height="120" width="120" alt="livebook" />
            </div>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <%# Note: it's fine to use stateless components in this comprehension,
                because @notebook_infos never change %>
            <%= for info <- @notebook_infos do %>
              <ExploreHelpers.notebook_card notebook_info={info} socket={@socket} />
            <% end %>
          </div>
        </div>
      </div>
    </div>

    <%= if @live_action == :user do %>
      <.current_user_modal
        return_to={Routes.explore_path(@socket, :page)}
        current_user={@current_user} />
    <% end %>
    """
  end

  @impl true
  def handle_params(%{"slug" => "new"}, _url, socket) do
    {:noreply, create_session(socket)}
  end

  def handle_params(%{"slug" => slug}, _url, socket) do
    {notebook, images} = Explore.notebook_by_slug!(slug)
    {:noreply, create_session(socket, notebook: notebook, images: images)}
  end

  def handle_params(_params, _url, socket), do: {:noreply, socket}
end
