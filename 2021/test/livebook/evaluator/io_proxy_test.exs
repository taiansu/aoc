defmodule Livebook.Evaluator.IOProxyTest do
  use ExUnit.Case, async: true

  alias Livebook.Evaluator
  alias Livebook.Evaluator.IOProxy

  setup do
    # {:ok, io} = IOProxy.start_link()

    {:ok, object_tracker} = start_supervised(Evaluator.ObjectTracker)
    {:ok, _pid, evaluator} = start_supervised({Evaluator, [object_tracker: object_tracker]})
    io = Process.info(evaluator.pid)[:group_leader]
    IOProxy.configure(io, self(), :ref)
    %{io: io}
  end

  describe ":stdio interoperability" do
    test "IO.puts", %{io: io} do
      IO.puts(io, "hey")
      assert_receive {:evaluation_output, :ref, "hey\n"}
    end

    test "IO.write", %{io: io} do
      IO.write(io, "hey")
      assert_receive {:evaluation_output, :ref, "hey"}
    end

    test "IO.inspect", %{io: io} do
      IO.inspect(io, %{}, [])
      assert_receive {:evaluation_output, :ref, "%{}\n"}
    end

    test "IO.read", %{io: io} do
      assert IO.read(io, :all) == {:error, :enotsup}
    end

    test "IO.gets", %{io: io} do
      assert IO.gets(io, "name: ") == {:error, :enotsup}
    end
  end

  describe "input" do
    test "responds to Livebook input request", %{io: io} do
      configure_owner_with_input(io, "input1", :value)

      assert livebook_get_input_value(io, "input1") == {:ok, :value}
    end

    test "responds to subsequent requests with the same value", %{io: io} do
      configure_owner_with_input(io, "input1", :value)

      assert livebook_get_input_value(io, "input1") == {:ok, :value}
      assert livebook_get_input_value(io, "input1") == {:ok, :value}
    end

    test "clear_input_cache/1 clears all cached input information", %{io: io} do
      pid =
        spawn_link(fn ->
          reply_to_input_request(:ref, "input1", {:ok, :value1}, 1)
          reply_to_input_request(:ref, "input1", {:ok, :value2}, 1)
        end)

      IOProxy.configure(io, pid, :ref)

      assert livebook_get_input_value(io, "input1") == {:ok, :value1}
      IOProxy.clear_input_cache(io)
      assert livebook_get_input_value(io, "input1") == {:ok, :value2}
    end
  end

  test "buffers rapid output", %{io: io} do
    IO.puts(io, "hey")
    IO.puts(io, "hey")
    assert_receive {:evaluation_output, :ref, "hey\nhey\n"}
  end

  test "respects CR as line cleaner", %{io: io} do
    IO.write(io, "hey")
    IO.write(io, "\roverride\r")
    assert_receive {:evaluation_output, :ref, "\roverride\r"}
  end

  test "flush/1 synchronously sends buffer contents", %{io: io} do
    IO.puts(io, "hey")
    IOProxy.flush(io)
    assert_received {:evaluation_output, :ref, "hey\n"}
  end

  test "supports direct livebook output forwarding", %{io: io} do
    livebook_put_output(io, {:text, "[1, 2, 3]"})

    assert_received {:evaluation_output, :ref, {:text, "[1, 2, 3]"}}
  end

  describe "token requests" do
    test "returns different tokens for subsequent calls", %{io: io} do
      IOProxy.configure(io, self(), :ref1)
      token1 = livebook_generate_token(io)
      token2 = livebook_generate_token(io)
      assert token1 != token2
    end

    test "returns different tokens for different refs", %{io: io} do
      IOProxy.configure(io, self(), :ref1)
      token1 = livebook_generate_token(io)
      IOProxy.configure(io, self(), :ref2)
      token2 = livebook_generate_token(io)
      assert token1 != token2
    end

    test "returns same tokens for the same ref", %{io: io} do
      IOProxy.configure(io, self(), :ref)
      token1 = livebook_generate_token(io)
      token2 = livebook_generate_token(io)
      IOProxy.configure(io, self(), :ref)
      token3 = livebook_generate_token(io)
      token4 = livebook_generate_token(io)
      assert token1 == token3
      assert token2 == token4
    end
  end

  # Helpers

  defp configure_owner_with_input(io, input_id, value) do
    pid =
      spawn_link(fn ->
        reply_to_input_request(:ref, input_id, {:ok, value}, 1)
      end)

    IOProxy.configure(io, pid, :ref)
  end

  defp reply_to_input_request(_ref, _input_id, _reply, 0), do: :ok

  defp reply_to_input_request(ref, input_id, reply, times) do
    receive do
      {:evaluation_input, ^ref, reply_to, ^input_id} ->
        send(reply_to, {:evaluation_input_reply, reply})
        reply_to_input_request(ref, input_id, reply, times - 1)
    end
  end

  defp livebook_put_output(io, output) do
    io_request(io, {:livebook_put_output, output})
  end

  defp livebook_get_input_value(io, input_id) do
    io_request(io, {:livebook_get_input_value, input_id})
  end

  defp livebook_generate_token(io) do
    io_request(io, :livebook_generate_token)
  end

  defp io_request(io, request) do
    ref = make_ref()
    send(io, {:io_request, self(), ref, request})
    assert_receive {:io_reply, ^ref, reply}
    reply
  end
end
