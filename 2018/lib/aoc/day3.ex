defmodule Aoc.Day3 do
  def part_1(file) do
    file
    |> File.stream!()
    |> Enum.map(&to_area/1)
    |> Enum.flat_map(&expand_to_pixels/1)
    |> Enum.group_by(& &1)
    |> Enum.filter(fn {_k, v} -> length(v) > 1 end)
    |> length
  end

  def to_area(line) do
    [c, x, y, w, h] =
      ~r/#(\d+) @ (\d+),(\d+): (\d+)x(\d+)/
      |> Regex.run(line, capture: :all_but_first)
      |> Enum.map(&String.to_integer/1)

    %{claim: c, from: {x, y}, width: w, height: h}
  end

  def expand_to_pixels(%{from: {x, y}, width: width, height: height}) do
    for w <- 0..(width - 1), h <- 0..(height - 1) do
      {x + w, y + h}
    end
  end

  def part_2(file) do
    claims =
      file
      |> File.stream!()
      |> Stream.map(&to_area/1)
      |> Stream.map(&claim_with_pixels/1)

    overlap_pixels =
      claims
      |> Stream.flat_map(& &1.pixels)
      |> Enum.group_by(& &1)
      |> Stream.filter(fn {_k, v} -> length(v) > 1 end)
      |> Stream.flat_map(fn {_k, v} -> v end)
      |> MapSet.new()

    Enum.find(claims, &no_overlap(&1, overlap_pixels))
  end

  def claim_with_pixels(%{claim: claim} = area) do
    %{claim: claim, pixels: expand_to_pixels(area)}
  end

  def no_overlap(l1, ms) do
    l1.pixels
    |> MapSet.new()
    |> MapSet.intersection(ms)
    |> MapSet.size()
    |> Kernel.==(0)
  end
end
