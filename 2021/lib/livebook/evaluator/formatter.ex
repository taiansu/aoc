defmodule Livebook.Evaluator.Formatter do
  @moduledoc false

  # Behaviour defining how evaluation results are transformed.
  #
  # The evaluation response is sent to the client as a message
  # and it may potentially be huge. If the client eventually
  # converts the result into some smaller representation,
  # we would unnecessarily send a lot of data.
  # By defining a custom formatter the client can instruct
  # the `Evaluator` to send already transformed data.
  #
  # Additionally if the results rely on external package installed
  # in the runtime node, then formatting anywhere else wouldn't be accurate,
  # for example using `inspect` on an external struct.

  alias Livebook.Evaluator

  @doc """
  Transforms the evaluation response.
  """
  @callback format_response(Evaluator.evaluation_response()) :: term()
end
