module Aoc.Day4

let data = System.IO.File.ReadAllLines "day4.txt"

type Note = int * int
type NotePair = Note * Note

let parseNote f (str: string) = str.Split "-" |> Array.map int |> f

let arrayToTwoTuple =
    function
    | [| a; b |] -> (a, b)
    | _ -> failwith "Invalid arrayToTwoTuple input"

let splitNote (str: string) =
    str.Split "-" |> Array.map int |> arrayToTwoTuple

let splitNotes (fst, snd) = (fst |> splitNote, snd |> splitNote)

let fullyCovered ((xStart, xEnd), (yStart, yEnd)) =
    (xStart <= yStart && yEnd <= xEnd) || (yStart <= xStart && xEnd <= yEnd)

let intersect (((xStart, xEnd), (yStart, yEnd)): NotePair) =
    let xSeq = seq { xStart..xEnd }
    let ySeq = seq { yStart..yEnd }
    Set.intersect (set xSeq) (set ySeq) |> Set.isEmpty |> not

let processedData: array<NotePair> =
    data
    |> Array.map (fun line -> line.Split ",")
    |> Array.map arrayToTwoTuple
    |> Array.map splitNotes

let first () =
    processedData |> Array.map fullyCovered |> Array.filter id |> Array.length

let second () =
    processedData |> Array.map intersect |> Array.filter id |> Array.length
