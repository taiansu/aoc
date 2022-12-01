module Aoc.Day1

let data = System.IO.File.ReadAllLines "day1.txt"

type Elf = int array
type Elfs = Elf array

let appendNum str acc =
    let num = int str
    Array.append acc [| num |]

let chunker (acc: Elfs) (calories: string) =
    match calories with
    | "" -> Array.append acc [| [||] |]
    | number ->
        let lastIndex = acc.Length - 1
        let last = acc |> Array.last
        Array.updateAt lastIndex (appendNum number last) acc

let first =
    fun () -> data |> Array.fold chunker [| [||] |] |> Array.map Array.sum |> Array.max


let second =
    fun () ->
        data
        |> Array.fold chunker [| [||] |]
        |> Array.map Array.sum
        |> Array.sortDescending
        |> Array.take 3
        |> Array.sum
