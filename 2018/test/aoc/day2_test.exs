defmodule Aoc.Day2Test do
  use ExUnit.Case

  test "part_a" do
    assert Aoc.Day2.part_a("./test/data/day2.txt") == 0
  end

  test "part_b" do
    assert Aoc.Day2.part_b("./test/data/day2.txt") == "pbykrmjmizwhxlqnasfgtycdv"
  end
end
