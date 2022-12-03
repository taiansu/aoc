module Aoc.Day3

let data = System.IO.File.ReadAllLines "day3.txt"

type rucksack = array<char>

let findInteract (a: seq<char>) (b: seq<char>) : seq<char> = Set.intersect (set a) (set b)

let findDup: (array<rucksack> -> char) =
    function
    | [| a; b |] -> findInteract a b |> Seq.head
    | n -> failwith $"Invalid input interact {n}"

let priority char : int =
    if 'A' <= char && char <= 'Z' then int char - 38
    else if 'a' <= char && char <= 'z' then int char - 96
    else failwith $"Invalid priority input {char}"

let first () =
    data
    |> Array.map (fun rucksack -> rucksack.ToCharArray())
    |> Array.map (Array.splitInto 2)
    |> Array.map findDup
    |> Array.map priority
    |> Array.sum

let findBadge: (array<rucksack> -> char) =
    function
    | [| a; b; c |] -> a |> findInteract b |> findInteract c |> Seq.head
    | n -> failwith $"Invalid input badge {n}"

let second () =
    data
    |> Array.map (fun rucksack -> rucksack.ToCharArray())
    |> Array.chunkBySize 3
    |> Array.map findBadge
    |> Array.map priority
    |> Array.sum
