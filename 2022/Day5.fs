module Aoc.Day5

open System.Text.RegularExpressions

let data = System.IO.File.ReadAllLines "day5.txt"

let (rawStacks, rawProcedures) =
    data |> Array.splitAt (Array.findIndex (fun line -> line = "") data)

let toStackList (stack: array<string>) =
    stack |> Array.toList |> List.filter (fun line -> line <> " ")

type Stacks = Map<string, list<string>>

let stacks: Stacks =
    rawStacks
    |> Array.map (fun line -> Regex.Matches(line, "\s?[ \[](\w|\s)[ \]]"))
    |> Array.map (Seq.map (fun m -> m.Groups[ 1 ].Captures[ 0 ].ToString()))
    |> Array.map (Seq.toArray)
    |> Array.transpose
    |> Array.map (fun line -> Array.splitAt (Array.length line - 1) line)
    |> Array.map (fun (stack, index) -> (Array.head index, stack |> toStackList))
    |> Map.ofArray

type Procedure = (int * string * string)
type Procedures = Procedure list

let listToTuple (list: list<string>) : Procedure =
    match list with
    | [ a; b; c ] -> (int a, b, c)
    | _ -> failwith $"Invalid listToTuple input: {list}"

let matchesToProcedures (matches: Match) : Procedure =
    matches.Groups
    |> Seq.map (fun i -> i.ToString())
    |> Seq.tail
    |> Seq.toList
    |> listToTuple

let procedures =
    rawProcedures
    |> Array.filter (fun line -> line <> "")
    |> Array.map (fun line -> Regex.Matches(line, "(\d+).*(\d+).*(\d+)"))
    |> Seq.collect (Seq.map matchesToProcedures)

let crateMover mover stacks procedure =
    let (number, f, t) = procedure
    let (take, rest) = Map.find f stacks |> List.splitAt number
    stacks |> Map.add f rest |> Map.add t (mover take @ Map.find t stacks)

let crateMover9000 = crateMover List.rev

let first () =
    Seq.fold crateMover9000 stacks procedures
    |> Map.values
    |> Seq.map List.head
    |> System.String.Concat

let crateMover9001 = crateMover id

let second () =
    Seq.fold crateMover9001 stacks procedures
    |> Map.values
    |> Seq.map List.head
    |> System.String.Concat
