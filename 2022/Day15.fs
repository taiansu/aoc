module Aoc.Day15

open System.IO
open System.Numerics
open System.Text.RegularExpressions

let data = File.ReadAllLines "day15.txt"

let sensorRgx =
    "Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"

type Point = int * int

type SensorInfo =
    { Sensor: Point
      Beacon: Point
      dist: int
      cover: Point -> bool }

let toSensorInfo sensorPair =
    let (sensor, beacon) = Seq.head sensorPair
    let dist = abs (fst sensor - fst beacon) + abs (snd sensor - snd beacon)

    let cover (x, y) =
        let dx = abs (fst sensor - x)
        let dy = abs (snd sensor - y)
        dx + dy <= dist

    { Sensor = sensor
      Beacon = beacon
      dist = dist
      cover = cover }

let sensors =
    data
    |> Array.map (fun line -> Regex.Matches(line, sensorRgx))
    |> Array.map (
        Seq.map (fun m ->
            (int m.Groups.[1].Value, int m.Groups.[2].Value), (int m.Groups.[3].Value, int m.Groups.[4].Value))
    )
    |> Array.map toSensorInfo

let xMin = sensors |> Array.map (fun s -> fst s.Sensor - s.dist) |> Array.min
let xMax = sensors |> Array.map (fun s -> fst s.Sensor + s.dist) |> Array.max

let height = 2000000

let impossible sensors point =
    printfn "Checking %A" point

    sensors |> Array.exists (fun s -> s.cover point || s.Sensor = point)
    && not (Array.exists (fun s -> s.Beacon = point) sensors)

let possible sensors point = not (impossible sensors point)

let manhattanCircle (r: int) ((x, y): Point) : array<Point> =
    seq { for i in 0..r -> (x + i, y + r - i) } |> Seq.toArray

// let first () = "only second"
let first () =
    seq { for x in xMin..xMax -> (x, height) }
    |> Seq.filter (impossible sensors)
    |> Seq.length

// Warning: this takes about 5 minutes to run
let second () =
    let range = 4000000

    sensors
    |> Seq.collect (fun s -> manhattanCircle (s.dist + 1) s.Sensor)
    |> Set.ofSeq
    |> Set.filter (fun (x, y) -> x >= 0 && x <= range && y >= 0 && y <= range)
    |> Seq.find (possible sensors)
    |> fun (x, y) -> (bigint x) * (bigint range) + (bigint y)
