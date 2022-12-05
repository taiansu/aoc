module Aoc.Day4

let data = System.IO.File.ReadAllLines "day4.txt"

type Note = int * int
type NotePair = Note * Note

let parseNote f (str: string) = str.Split "-" |> Array.map int |> f

let arrayToPair =
    function
    | [| a; b |] -> (a, b)
    | _ -> failwith "Invalid arrayToPair input"

let splitNote (str: string) =
    str.Split "-" |> Array.map int |> arrayToPair

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
    |> Array.map arrayToPair
    |> Array.map splitNotes

let first () =
    processedData |> Array.map fullyCovered |> Array.filter id |> Array.length

let second () =
    processedData |> Array.map intersect |> Array.filter id |> Array.length
