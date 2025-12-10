import gleam/int
import gleam/io
import gleam/list
import gleam/option.{None, Some}
import gleam/set.{type Set}
import gleam/string
import simplifile
import utils

fn read_input_to_lines() -> List(String) {
  let filename = "inputs/day10.txt"
  case simplifile.read(filename) {
    Ok(content) -> content |> string.trim |> string.split(on: "\n")
    Error(_) -> []
  }
}

type Lights =
  List(String)

type Button =
  Set(Int)

type Joltage =
  List(String)

fn parse_line(line: String) -> #(Lights, List(Button), Joltage) {
  let parts = line |> string.split(" ")
  parts
  |> list.fold(#([], [], []), fn(acc, pt) {
    let #(target, bs, j) = acc
    case pt {
      "[" <> t -> #(t |> string.drop_end(1) |> string.to_graphemes, bs, j)
      "(" <> t -> #(
        target,
        [
          t
            |> string.drop_end(1)
            |> string.split(",")
            |> list.filter_map(int.parse)
            |> set.from_list,
          ..bs
        ],
        j,
      )
      "{" <> t -> #(target, bs, t |> string.drop_end(1) |> string.split(","))
      _ -> panic as "not valid input"
    }
  })
}

fn apply_button(init: Lights, bt: Button) -> Lights {
  init
  |> list.index_map(fn(s, index) {
    case bt |> set.contains(index) {
      False -> s
      True ->
        case s {
          "#" -> "."
          "." -> "#"
          _ -> panic as "not valid light"
        }
    }
  })
}

pub fn puzzle0() -> Nil {
  let input = read_input_to_lines() |> list.map(parse_line)
  let ret =
    input
    |> list.map(fn(item) {
      let #(target, buttons, _) = item
      let len = target |> list.length
      let init = "." |> list.repeat(len)
      find_min(target, init, buttons)
    })
    |> utils.list_sum

  io.println("aco25::day10::puzzle0 " <> int.to_string(ret))
}

fn find_min(
  target: List(String),
  init: List(String),
  buttons: List(Set(Int)),
) -> Int {
  let len = buttons |> list.length
  let assert Some(ret) =
    list.range(1, len)
    |> list.fold(None, fn(acc, index) {
      case acc {
        None -> {
          case
            buttons
            |> list.combinations(index)
            |> list.any(fn(bts) {
              bts |> list.fold(init, fn(acc, rule) { apply_button(acc, rule) })
              == target
            })
          {
            False -> None
            True -> Some(index)
          }
        }
        Some(x) -> Some(x)
      }
    })
  ret
}

pub fn puzzle1() -> Nil {
  let input = read_input_to_lines() |> list.map(parse_line)
  let ret = 0

  io.println("aco25::day10::puzzle1 " <> int.to_string(ret))
}
