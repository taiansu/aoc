module Aoc.Day6

let data = System.IO.File.ReadAllLines "day6.txt"

let chunker (len: int) (input: array<char>) accu index =
    let chunk = input[index .. (index + len - 1)]
    Array.append accu [| chunk |]

let chunkWithStep len (input: array<char>) =
    [ 0 .. input.Length - 1 ] |> Seq.fold (chunker len input) [||]

let findStartMark len data =
    data
    |> Array.head
    |> Seq.toArray
    |> chunkWithStep len
    |> Array.map (fun c -> c |> set |> Set.count = c.Length)
    |> Array.findIndex id
    |> (+) len

let first = fun () -> findStartMark 4 data

let second = fun () -> findStartMark 14 data
