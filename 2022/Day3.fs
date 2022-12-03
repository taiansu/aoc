module Aoc.Day3

let data = System.IO.File.ReadAllLines "day3.txt"

type Rucksack = array<char>

let intersect (a: seq<char>) (b: seq<char>) : seq<char> = Set.intersect (set a) (set b)

let findDup: (array<Rucksack> -> char) =
    function
    | [| a; b |] -> intersect a b |> Seq.head
    | n -> failwith $"Invalid findDup input: {n}"

let priority char : int =
    if 'A' <= char && char <= 'Z' then int char - 38
    elif 'a' <= char && char <= 'z' then int char - 96
    else failwith $"Invalid priority input: {char}"

let first () =
    data
    |> Array.map (fun rucksack -> rucksack.ToCharArray())
    |> Array.map (Array.splitInto 2)
    |> Array.map findDup
    |> Array.map priority
    |> Array.sum

let findBadge: (array<Rucksack> -> char) =
    function
    | [| a; b; c |] -> a |> intersect b |> intersect c |> Seq.head
    | n -> failwith $"Invalid findBadge input: {n}"

let second () =
    data
    |> Array.map (fun rucksack -> rucksack.ToCharArray())
    |> Array.chunkBySize 3
    |> Array.map findBadge
    |> Array.map priority
    |> Array.sum
