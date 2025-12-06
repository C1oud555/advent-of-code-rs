import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

import utils

fn parse_input() -> #(List(String), List(List(Int))) {
  let filename = "inputs/day6.txt"
  case simplifile.read(filename) {
    Ok(content) -> {
      let lines = content |> string.trim |> string.split(on: "\n")
      case lines |> list.reverse {
        [head, ..tail] -> {
          #(
            head
              |> string.trim
              |> string.split(" ")
              |> list.filter(fn(op) { op |> string.is_empty |> bool.negate }),
            tail
              |> list.map(fn(line) {
                line
                |> string.trim
                |> string.split(" ")
                |> list.filter_map(int.parse)
              }),
          )
        }
        _ -> panic as "invalid input"
      }
    }
    Error(_) -> panic as "invalid input"
  }
}

pub fn puzzle0() -> Nil {
  let #(ops, operands) = parse_input()

  let operands = operands |> list.transpose

  let ret =
    ops
    |> list.zip(operands)
    |> list.map(fn(item) {
      let #(op, operand) = item
      {
        case op {
          "+" -> operand |> utils.list_sum
          "*" -> operand |> utils.list_product
          _ -> panic as { "invalid op" <> op }
        }
      }
    })
    |> utils.list_sum

  io.println("aco25::day6::puzzle0 " <> int.to_string(ret))
}

pub fn puzzle1() -> Nil {
  let #(ops, operands) = parse_input1()

  let ret =
    ops
    |> list.zip(operands)
    |> list.map(fn(item) {
      let #(op, operand) = item
      {
        case op {
          "+" -> operand |> utils.list_sum
          "*" -> operand |> utils.list_product
          _ -> panic as { "invalid op" <> op }
        }
      }
    })
    |> utils.list_sum
  io.println("aco25::day6::puzzle1 " <> int.to_string(ret))
}

fn parse_input1() -> #(List(String), List(List(Int))) {
  let filename = "inputs/day6.txt"
  case simplifile.read(filename) {
    Ok(content) -> {
      let lines = content |> string.trim |> string.split(on: "\n")
      let lines =
        lines |> list.map(string.to_graphemes) |> list.transpose |> list.reverse

      let #(ops, operands, _) =
        lines
        |> list.fold(#([], [], []), fn(acc, item) {
          let #(ops, operands, parsed_nums) = acc
          let len = item |> list.length
          let #(init, last) = item |> list.split(len - 1)
          case last {
            ["*"] -> {
              let assert Ok(num) =
                init |> string.concat |> string.trim |> int.parse
              let new_operands = [num, ..parsed_nums]
              #(["*", ..ops], [new_operands, ..operands], [])
            }
            ["+"] -> {
              let assert Ok(num) =
                init |> string.concat |> string.trim |> int.parse
              let new_operands = [num, ..parsed_nums]
              #(["+", ..ops], [new_operands, ..operands], [])
            }
            _ -> {
              let num_result = item |> string.concat |> string.trim |> int.parse
              case num_result {
                Ok(num) -> #(ops, operands, [num, ..parsed_nums])
                Error(_) -> acc
              }
            }
          }
        })
      #(ops, operands)
    }
    Error(_) -> panic as "invalid input"
  }
}
