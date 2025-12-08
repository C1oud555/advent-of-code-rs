import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/string
import simplifile

import utils

type Coord =
  #(Int, Int, Int)

fn distance(l: Coord, r: Coord) -> Int {
  let #(l0, l1, l2) = l
  let #(r0, r1, r2) = r
  let #(d0, d1, d2) = #(r0 - l0, r1 - l1, r2 - l2)
  d0 * d0 + d1 * d1 + d2 * d2
}

fn parse_input() -> List(Coord) {
  let filename = "inputs/day8.txt"
  case simplifile.read(filename) {
    Ok(content) -> {
      content
      |> string.trim
      |> string.split(on: "\n")
      |> list.map(fn(line) {
        let nums = line |> string.split(on: ",") |> list.map(int.parse)
        let assert [Ok(num0), Ok(num1), Ok(num2)] = nums
        #(num0, num1, num2)
      })
    }
    Error(_) -> panic as "invalid input"
  }
}

pub fn puzzle0() -> Nil {
  let input = parse_input()
  let distances =
    input
    |> list.combinations(2)
    |> list.map(fn(item) {
      let assert [num0, num1] = item
      #(num0, num1, distance(num0, num1))
    })
    |> list.sort(fn(l, r) {
      let #(_, _, f0) = l
      let #(_, _, f1) = r
      int.compare(f0, f1)
    })

  let input_len = distances |> list.length

  let #(merged, _) =
    distances
    |> list.take(1000)
    |> do_merge(input_len)

  let ret =
    merged
    |> list.map(fn(item) { item |> list.length })
    |> list.sort(int.compare)
    |> list.reverse
    |> list.take(3)
    |> utils.list_product

  io.println("aco25::day8::puzzle0 " <> int.to_string(ret))
}

fn do_merge(
  l: List(#(Coord, Coord, Int)),
  input_len: Int,
) -> #(List(List(Coord)), Option(Int)) {
  l
  |> list.fold(#([], None), fn(acc, item) {
    let #(num0, num1, _) = item

    let #(acc, ff) = acc

    let grouped_list =
      acc
      |> list.group(fn(line) {
        line |> list.contains(num0) || line |> list.contains(num1)
      })
      |> dict.to_list

    let tmp = case grouped_list {
      [#(False, not_contain), #(True, contain)] -> {
        case contain {
          [line] -> {
            case line |> list.contains(num0), line |> list.contains(num1) {
              True, True -> acc
              True, False -> {
                [[num1, ..line], ..not_contain]
              }
              False, True -> {
                [[num0, ..line], ..not_contain]
              }
              False, False -> panic as "should not happend"
            }
          }
          [line0, line1] -> {
            [line0 |> list.append(line1), ..not_contain]
          }
          _ -> panic as { "should not happen" }
        }
      }
      [#(True, contain)] -> {
        case contain {
          [line] -> {
            case line |> list.contains(num0), line |> list.contains(num1) {
              True, True -> acc
              True, False -> {
                [[num1, ..line]]
              }
              False, True -> {
                [[num0, ..line]]
              }
              False, False -> panic as "should not happend"
            }
          }
          [line0, line1] -> {
            [line0 |> list.append(line1)]
          }
          _ -> panic as { "should not happen" }
        }
      }
      _ -> {
        [[num0, num1], ..acc]
      }
    }
    case ff {
      None ->
        case tmp {
          [hh] -> {
            case hh |> list.length == input_len {
              True -> {
                let #(x0, _, _) = num0
                let #(x1, _, _) = num1
                #(tmp, Some(x0 * x1))
              }
              False -> {
                #(tmp, None)
              }
            }
          }
          _ -> {
            #(tmp, None)
          }
        }
      Some(_) -> #(tmp, ff)
    }
  })
}

pub fn puzzle1() -> Nil {
  let input = parse_input()
  let distances =
    input
    |> list.combinations(2)
    |> list.map(fn(item) {
      let assert [num0, num1] = item
      #(num0, num1, distance(num0, num1))
    })
    |> list.sort(fn(l, r) {
      let #(_, _, f0) = l
      let #(_, _, f1) = r
      int.compare(f0, f1)
    })

  let input_len = input |> list.length

  let assert #(_, Some(ret)) = distances |> do_merge(input_len)

  io.println("aco25::day8::puzzle1 " <> int.to_string(ret))
}
