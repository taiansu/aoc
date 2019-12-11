defmodule Aoc.Day3Test do
  use ExUnit.Case

  test "part_1" do
    assert Aoc.Day3.part_1("./test/data/day3.txt") == 0
  end

  @tag timeout: :infinity
  test "part_2" do
    assert Aoc.Day3.part_2("./test/data/day3.txt") == 0
  end
end
