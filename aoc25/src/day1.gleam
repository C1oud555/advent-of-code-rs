import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

fn read_input_to_lines() -> List(String) {
  let filename = "inputs/day1.txt"
  case simplifile.read(filename) {
    Ok(content) -> string.split(content, on: "\n")
    Error(_) -> []
  }
}

fn parse_line(line: String) -> #(String, Int) {
  case line {
    "L" <> num -> #("L", int.parse(num) |> result.unwrap(-10))
    "R" <> num -> #("R", int.parse(num) |> result.unwrap(-10))
    _ -> #("T", 0)
  }
}

pub fn puzzle0() -> Nil {
  let input = read_input_to_lines()
  let #(_, zero_cnt) =
    input
    |> list.fold(#(50, 0), fn(acc, line) {
      let #(st, zcnt) = acc
      let #(direction, num) = parse_line(line)
      let nst = case direction {
        "L" -> { st + 100 - num } % 100
        "R" -> { st + 100 + num } % 100
        _ -> st
      }
      let nzcnt =
        zcnt
        + case nst {
          0 -> 1
          _ -> 0
        }
      #(nst, nzcnt)
    })

  io.println("aco25::puzzle0 " <> zero_cnt |> int.to_string)
}

pub fn rotate(st: Int, zcnt: Int, num: Int, direction: String) -> #(Int, Int) {
  case num {
    0 -> #(st, zcnt)
    _ -> {
      let nst = case direction {
        "L" -> { st + 100 - 1 } % 100
        "R" -> { st + 100 + 1 } % 100
        _ -> st
      }
      let nzcnt =
        zcnt
        + case nst {
          0 -> 1
          _ -> 0
        }
      rotate(nst, nzcnt, num - 1, direction)
    }
  }
}

pub fn puzzle1() -> Nil {
  let input = read_input_to_lines()
  let #(_, zero_cnt) =
    input
    |> list.fold(#(50, 0), fn(acc, line) {
      let #(st, zcnt) = acc
      let #(direction, num) = parse_line(line)
      rotate(st, zcnt, num, direction)
    })

  io.println("aco25::puzzle1 " <> zero_cnt |> int.to_string)
}
