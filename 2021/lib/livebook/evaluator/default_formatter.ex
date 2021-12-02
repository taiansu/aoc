defmodule Livebook.Evaluator.DefaultFormatter do
  @moduledoc false

  # The formatter used by Livebook for rendering the results.
  #
  # See `Livebook.Notebook.Cell` for available output formats.

  @behaviour Livebook.Evaluator.Formatter

  require Logger

  @impl true
  def format_response({:ok, :"do not show this result in output"}) do
    # Functions in the `IEx.Helpers` module return this specific value
    # to indicate no result should be printed in the iex shell,
    # so we respect that as well.
    :ignored
  end

  def format_response({:ok, {:module, _, _, _} = value}) do
    to_inspect_output(value, limit: 10)
  end

  def format_response({:ok, value}) do
    to_output(value)
  end

  def format_response({:error, kind, error, stacktrace}) do
    formatted = Exception.format(kind, error, stacktrace)
    {:error, formatted, error_type(error)}
  end

  @compile {:no_warn_undefined, {Kino.Render, :to_livebook, 1}}

  defp to_output(value) do
    # Kino is a "client side" extension for Livebook that may be
    # installed into the runtime node. If it is installed we use
    # its more precies output rendering rules.
    if Code.ensure_loaded?(Kino.Render) do
      try do
        Kino.Render.to_livebook(value)
      catch
        kind, error ->
          {error, stacktrace} = Exception.blame(kind, error, __STACKTRACE__)
          formatted = Exception.format(kind, error, stacktrace)
          Logger.error(formatted)
          to_inspect_output(value)
      end
    else
      to_inspect_output(value)
    end
  end

  defp to_inspect_output(value, opts \\ []) do
    inspected = inspect(value, inspect_opts(opts))
    {:text, inspected}
  end

  defp inspect_opts(opts) do
    default_opts = [pretty: true, width: 100, syntax_colors: syntax_colors()]
    Keyword.merge(default_opts, opts)
  end

  defp syntax_colors() do
    # Note: we intentionally don't specify colors
    # for `:binary`, `:list`, `:map` and `:tuple`
    # and rely on these using the default text color.
    # This way we avoid a bunch of HTML tags for coloring commas, etc.
    [
      atom: :blue,
      # binary: :light_black,
      boolean: :magenta,
      # list: :light_black,
      # map: :light_black,
      number: :blue,
      nil: :magenta,
      regex: :red,
      string: :green,
      # tuple: :light_black,
      reset: :reset
    ]
  end

  defp error_type(error) do
    cond do
      mix_install_vm_error?(error) -> :runtime_restart_required
      true -> :other
    end
  end

  defp mix_install_vm_error?(exception) do
    is_struct(exception, Mix.Error) and
      Exception.message(exception) =~
        "Mix.install/2 can only be called with the same dependencies"
  end
end
