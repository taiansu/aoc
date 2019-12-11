defmodule Aoc.Day6 do
  @doc """
  Parse coordinate

  ## Examples

      iex> Aoc.Day6.parse_coordinate("1, 3")
      {1, 3}
  """
  def parse_coordinate(binary) when is_binary(binary) do
    [x, y] = String.split(binary, ", ")
    {String.to_integer(x), String.to_integer(y)}
  end

  @doc """
  Get the bounding box for a list of coordinates

  ## Examples

      iex> Aoc.Day6.bounding_box([
      ...>   {1, 1},
      ...>   {1, 6},
      ...>   {8, 3},
      ...>   {3, 4},
      ...>   {5, 5},
      ...>   {8, 9}
      ...> ])
      {1..8, 1..9}
  """
  def bounding_box(coordinates) do
    {{min_x, _}, {max_x, _}} = Enum.min_max_by(coordinates, &elem(&1, 0))
    {{_, min_y}, {_, max_y}} = Enum.min_max_by(coordinates, &elem(&1, 1))

    {min_x..max_x, min_y..max_y}
  end

  @doc """
  Get a grid of all the cloest coordinates to each point

  ## Examples

      iex> Aoc.Day6.closest_grid([{1, 1}, {3, 3}], 1..3, 1..3)
      %{
        {1, 1} => {1, 1},
        {1, 2} => {1, 1},
        {1, 3} => nil,
        {2, 1} => {1, 1},
        {2, 2} => nil,
        {2, 3} => {3, 3},
        {3, 1} => nil,
        {3, 2} => {3, 3},
        {3, 3} => {3, 3}
      }
  """
  def closest_grid(coordinates, x_range, y_range) do
    for x <- x_range,
        y <- y_range,
        point = {x, y},
        do: {point, cloest_coordinate(coordinates, point)},
        into: %{}
  end

  @doc """
  Find the largest area

  ## Example

      iex> Aoc.Day6.largest_area([
      ...> {1, 1},
      ...> {1, 6},
      ...> {8, 3},
      ...> {3, 4},
      ...> {5, 5},
      ...> {8, 9}
      ...>])
      17
  """
  def largest_area(coordinates) do
    infinite_coordinates = infinite_coordinates(coordinates)

    {x_range, y_range} = bounding_box(coordinates)
    closest_grid = closest_grid(coordinates, x_range, y_range)

    finite_count =
      Enum.reduce(closest_grid, %{}, fn {_, coordinate}, acc ->
        if is_nil(coordinate) or coordinate in infinite_coordinates do
          acc
        else
          Map.update(acc, coordinate, 1, &(&1 + 1))
        end
      end)

    finite_count
    |> Enum.max_by(fn {_coordinate, count} -> count end)
    |> elem(1)
  end

  defp cloest_coordinate(coordinates, point) do
    coordinates
    |> Enum.map(&{manhattan_distance(&1, point), &1})
    |> Enum.sort()
    |> case do
      [{0, coordinate} | _] -> coordinate
      [{distance, _}, {distance, _} | _] -> nil
      [{_, coordinate} | _] -> coordinate
    end
  end

  def infinite_coordinates(coordinates) do
    {x_range, y_range} = bounding_box(coordinates)
    closest_grid = closest_grid(coordinates, x_range, y_range)

    infinite_for_x =
      for y <- [y_range.first, y_range.last],
          x <- x_range,
          do: closest_grid[{x, y}]

    infinite_for_y =
      for x <- [x_range.first, x_range.last],
          y <- y_range,
          do: closest_grid[{x, y}]

    MapSet.new(infinite_for_x ++ infinite_for_y)
  end

  @doc """
  Calculate the size of region which has sum of distance to all coordinates under the limit

  ## Example

  iex> Aoc.Day6.size_of_region([
  ...>   {1, 1},
  ...>   {1, 6},
  ...>   {8, 3},
  ...>   {3, 4},
  ...>   {5, 5},
  ...>   {8, 9}
  ...> ], 32)
  16
  """
  def size_of_region(coordinates, limit) do
    {x_range, y_range} = bounding_box(coordinates)

    matrix =
      for x <- x_range,
          y <- y_range,
          point = {x, y},
          do: sum_of_distance(point, coordinates)

    matrix
    |> Enum.filter(fn d -> d < limit end)
    |> Enum.count()
  end

  def sum_of_distance(point, coordinates) do
    coordinates
    |> Enum.map(&manhattan_distance(&1, point))
    |> Enum.sum()
  end

  def manhattan_distance({x1, y1}, {x2, y2}) do
    abs(x1 - x2) + abs(y1 - y2)
  end
end
