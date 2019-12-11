defmodule Aoc.Day2 do
  def part_a(file) do
    file
    |> File.stream!()
    |> Stream.map(&group_by_occurrence/1)
    |> Stream.flat_map(&only_2_or_3/1)
    |> length_of_occurance()
    |> Enum.to_list()
    |> (fn l -> apply(Kernel, :*, l) end).()
  end

  defp group_by_occurrence(id) do
    id
    |> String.split("", trim: true)
    |> length_of_occurance()
  end

  defp length_of_occurance(list) do
    list
    |> Enum.sort()
    |> Stream.chunk_by(& &1)
    |> Stream.map(&length/1)
  end

  defp only_2_or_3(list) do
    list
    |> Enum.filter(&(&1 == 2 || &1 == 3))
    |> Enum.dedup()
  end

  def part_b(file) do
    file
    |> File.stream!()
    |> Stream.map(&String.split(&1, "", trim: true))
    |> Enum.reduce_while([], &find_one_off/2)
    |> remove_diff()
    |> Enum.join()
    |> String.trim()
  end

  def find_one_off(l, accu) do
    if member = Enum.find(accu, &one_char_diff?(&1, l)) do
      {:halt, {member, l}}
    else
      {:cont, [l | accu]}
    end
  end

  def one_char_diff?(l1, l2) do
    Enum.zip(l1, l2)
    |> Enum.reject(&same_char/1)
    |> Enum.count()
    |> (fn i -> i == 1 end).()
  end

  def same_char({x, y}), do: x == y

  def remove_diff({l1, l2}) do
    l1
    |> Enum.zip(l2)
    |> Enum.filter(&same_char/1)
    |> Enum.unzip()
    |> (fn l -> elem(l, 0) end).()
  end
end
