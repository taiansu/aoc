module Aoc.Day2

let data = System.IO.File.ReadAllLines "day2.txt"

type Game = string array

let games = data |> Array.map (fun line -> line.Split [| ' ' |])

let match_score1 =
    function
    | [| "A"; "Z" |]
    | [| "B"; "X" |]
    | [| "C"; "Y" |] -> 0
    | [| "A"; "X" |]
    | [| "B"; "Y" |]
    | [| "C"; "Z" |] -> 3
    | [| "A"; "Y" |]
    | [| "B"; "Z" |]
    | [| "C"; "X" |] -> 6
    | _ -> 0

let shape_score1 =
    function
    | [| _; "X" |] -> 1
    | [| _; "Y" |] -> 2
    | [| _; "Z" |] -> 3
    | _ -> 0

let game_score match_score_fn shape_score_fn (game: Game) =
    match_score_fn game + shape_score_fn game

let first () =
    games |> Array.map (game_score match_score1 shape_score1) |> Array.sum

let match_score2 =
    function
    | [| _; "X" |] -> 0
    | [| _; "Y" |] -> 3
    | [| _; "Z" |] -> 6
    | _ -> 0

let shape_score2 =
    function
    | [| "A"; "Y" |]
    | [| "B"; "X" |]
    | [| "C"; "Z" |] -> 1
    | [| "A"; "Z" |]
    | [| "B"; "Y" |]
    | [| "C"; "X" |] -> 2
    | [| "A"; "X" |]
    | [| "B"; "Z" |]
    | [| "C"; "Y" |] -> 3
    | _ -> 0

let second () =
    games |> Array.map (game_score match_score2 shape_score2) |> Array.sum
