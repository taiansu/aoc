module Aoc.Day10

open System.IO

let data = File.ReadAllLines "day10.txt"

type Ins = Ins

let (|Noop|_|) (Ins, (cmd: string)) = if cmd = "noop" then Some(0) else None

let (|Addx|_|) (Ins, (cmd: string)) =
    if cmd.StartsWith("addx") then Some(int cmd[5..]) else None

let expandPositions (instruction: string) : array<int> =
    match (Ins, instruction) with
    | Noop pos -> [| pos |]
    | Addx pos -> [| 0; pos |]
    | _ -> failwith $"unknown instruction {instruction}"

let instructions = data |> Array.collect expandPositions

let toSignal (cycle: int) (data: array<int>) : int =
    data |> Array.take (cycle - 1) |> Array.sum |> (+) 1

let toSignalStrength (data: array<int>) (cycle: int) : int = data |> toSignal cycle |> (*) cycle

let toChar (draw: bool) : char = if draw then '#' else ' '

let first () =
    seq { 20..40..220 } |> Seq.map (toSignalStrength instructions) |> Seq.sum

let inSpritePostion (instuctions: array<int>) (cycle: int) : bool =
    instructions
    |> toSignal (cycle + 1)
    |> (fun x -> (cycle % 40, [| x - 1; x; x + 1 |]))
    ||> Array.contains

let second () =
    seq { 0..240 }
    |> Seq.map (inSpritePostion instructions)
    |> Seq.map toChar
    |> Seq.chunkBySize 40
    |> Seq.map Seq.toArray
    |> Seq.map System.String
    |> Seq.map (printfn "%s")
    |> Seq.toArray
