defmodule Aoc.Day1 do
  def part1_exec(file) do
    file
    |> File.stream!()
    |> Enum.reduce(0, &calc/2)
  end

  def part2_exec(file) do
    file
    |> File.stream!()
    |> Stream.cycle()
    |> Stream.scan(0, &calc/2)
    |> Enum.reduce_while([], &find_dup/2)
  end

  def calc(str, accu) do
    str
    |> String.trim("\n")
    |> String.to_integer()
    |> Kernel.+(accu)
  end

  def find_dup(next, accu) do
    # IO.inspect next
    if Enum.member?(accu, next) do
      {:halt, next}
    else
      {:cont, [next | accu]}
    end
  end
end
