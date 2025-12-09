import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/string
import simplifile

fn parse_input() -> List(#(Int, Int)) {
  let filename = "inputs/day9.txt"
  case simplifile.read(filename) {
    Ok(content) -> {
      content
      |> string.trim
      |> string.split(on: "\n")
      |> list.map(fn(line) {
        let nums = line |> string.split(on: ",") |> list.map(int.parse)
        let assert [Ok(num0), Ok(num1)] = nums
        #(num0, num1)
      })
    }
    Error(_) -> panic as "invalid input"
  }
}

fn area(pair: List(#(Int, Int))) -> #(Int, #(Int, Int), #(Int, Int)) {
  let assert [#(a0, b0), #(a1, b1)] = pair
  let h = { a1 - a0 } |> int.absolute_value
  let w = { b1 - b0 } |> int.absolute_value
  #({ h + 1 } * { w + 1 }, #(a0, b0), #(a1, b1))
}

pub fn puzzle0() -> Nil {
  let input = parse_input()
  let assert Ok(ret) =
    input
    |> list.combinations(2)
    |> list.map(area)
    |> list.map(fn(item) { item.0 })
    |> list.max(int.compare)

  io.println("aco25::day9::puzzle0 " <> int.to_string(ret))
}

fn build_range(input: List(#(Int, Int))) -> List(#(Int, Int)) {
  let assert Ok(start) = input |> list.last
  let #(edge_points, _) =
    input
    |> list.fold(#([], start), fn(acc, item) {
      let #(points, #(x0, y0)) = acc
      let #(x1, y1) = item
      let new_set = case x0 == x1, y0 == y1 {
        True, False -> {
          list.range(y0, y1)
          |> list.map(fn(item) { #(x0, item) })
          |> list.append(points)
        }
        False, True -> {
          list.range(x0, x1)
          |> list.map(fn(item) { #(item, y0) })
          |> list.append(points)
        }
        _, _ -> panic as "should not happend"
      }
      #(new_set, item)
    })

  edge_points
}

pub fn puzzle1() -> Nil {
  let input = parse_input()

  let edge_points = build_range(input)

  let combinations =
    input
    |> list.combinations(2)
    |> list.map(area)
    |> list.sort(fn(l, r) { int.compare(l.0, r.0) })
    |> list.reverse()

  let assert Ok(#(ret, _, _)) =
    combinations
    |> list.find(fn(item) {
      let #(_, p0, p1) = item
      let #(x0, y0) = #(int.min(p0.0, p1.0), int.max(p0.0, p1.0))
      let #(x1, y1) = #(int.min(p0.1, p1.1), int.max(p0.1, p1.1))

      let ps0 =
        edge_points
        |> list.filter(fn(t) {
          let #(x, y) = t
          x == x0 && y < y1 && y > y1
        })
      let ps1 =
        edge_points
        |> list.filter(fn(t) {
          let #(x, y) = t
          x == x1 && y < y1 && y > y1
        })
      let ps2 =
        edge_points
        |> list.filter(fn(t) {
          let #(x, y) = t
          y == y0 && x < x1 && x > x0
        })
      let ps3 =
        edge_points
        |> list.filter(fn(t) {
          let #(x, y) = t
          y == y1 && x < x1 && x > x0
        })

      [ps0, ps1, ps2, ps3]
      |> list.all(fn(item) { { item |> list.length } != 1 })
    })

  io.println("aco25::day9::puzzle1 " <> int.to_string(ret))
}
