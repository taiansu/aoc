module Aoc.Day1

let data = System.IO.File.ReadAllLines "day1.txt"

let chunker (acc: array<int>) (calories: string) =
    match calories with
    | "" -> Array.append acc [| 0 |]
    | number ->
        let lastIndex = acc.Length - 1
        let last = Array.last acc
        Array.updateAt lastIndex (int number + last) acc

let first = fun () -> data |> Array.fold chunker [| 0 |] |> Array.max

let second =
    fun () ->
        data
        |> Array.fold chunker [| 0 |]
        |> Array.sortDescending
        |> Array.take 3
        |> Array.sum
