module Aoc.Day7

open System
open System.IO

let data = File.ReadAllLines "day7.txt"

type Routes = array<string>
type Du = Map<string, int>

let private combinePath (accu: string) (d: string) = [| accu; d |] |> Path.Combine

let private ancestorDirs routes =
    routes |> Array.rev |> Array.scan combinePath "" |> Array.tail

let private updateDu (size: int) (accu: Du) name =
    Map.change
        name
        (fun i ->
            match i with
            | Some number -> Some(number + size)
            | None -> failwith $"not found {i}")
        accu

let subDir (routes: Routes) (name: string) =
    routes |> Array.append [| name |] |> Array.rev |> Path.Combine

// Active Pattern
type Line = Line

let (|Cd|_|) (Line, (cmd: string)) =
    if cmd.StartsWith("$ cd ") then Some(cmd[5..]) else None

let (|Dir|_|) (Line, (cmd: string)) =
    if cmd.StartsWith("dir ") then Some(cmd[4..]) else None

let (|File|_|) (Line, (cmd: string)) =
    let first_char = cmd.[0]

    if '0' <= first_char && first_char <= '9' then
        let size = cmd.Split(' ', 2, StringSplitOptions.None) |> Array.head
        Some(int size)
    else
        None

let run (routes: Routes, du: Du) (line: string) =
    match (Line, line) with
    | Cd "/" -> ([| "/" |], du)
    | Cd ".." -> (Array.tail routes, du)
    | Cd dir -> (Array.append [| dir |] routes, du)
    | Dir name -> (routes, Map.add (subDir routes name) 0 du)
    | File size -> (routes, routes |> ancestorDirs |> Array.fold (updateDu size) du)
    | _ -> (routes, du)

let du = data |> Array.fold run ([| "/" |], Map [ "/", 0 ]) |> snd

let first () =
    du |> Map.filter (fun k v -> v <= 100000) |> Map.values |> Seq.sum

let second () =
    let unusedSpace = 70000000 - (Map.find "/" du)
    let requiredSpace = 30000000 - unusedSpace
    du |> Map.filter (fun k v -> v >= requiredSpace) |> Map.values |> Seq.min
