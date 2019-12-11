defmodule Aoc.Day1Test do
  use ExUnit.Case
  alias Aoc.Day1

  test "part 1" do
    assert Day1.part1_exec("./test/data/day1.txt") == 0
  end

  @tag timeout: :infinity
  test "part 2" do
    assert Day1.part2_exec("./test/data/day1.txt") == 0
  end
end
