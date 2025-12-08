import gleam/dict
import gleam/int
import gleam/io
import gleam/list
import gleam/option.{None, Some}
import gleam/string
import simplifile

import utils

fn parse_input() -> List(String) {
  let filename = "inputs/day7.txt"
  case simplifile.read(filename) {
    Ok(content) -> {
      content |> string.trim |> string.split(on: "\n")
    }
    Error(_) -> panic as "invalid input"
  }
}

pub fn puzzle0() -> Nil {
  let input = parse_input()
  let assert [head, ..tail] = input

  let init_poses =
    head
    |> string.to_graphemes
    |> utils.index_of(fn(item1) { item1 == "S" })

  let #(ret, _) =
    tail
    |> list.fold(#(0, init_poses), fn(acc, item) {
      let splitter_poses =
        item
        |> string.to_graphemes
        |> utils.index_of(fn(item1) { item1 == "^" })

      let #(last_split_cnt, last_beam_pos) = acc
      let #(nsc, nbp) =
        last_beam_pos
        |> list.fold(#(0, []), fn(acc1, item1) {
          let #(new_split_cnt, new_beam_pos) = acc1
          case splitter_poses |> list.contains(item1) {
            True -> #(new_split_cnt + 1, [item1 - 1, item1 + 1, ..new_beam_pos])
            False -> #(new_split_cnt, [item1, ..new_beam_pos])
          }
        })
      #(last_split_cnt + nsc, nbp |> list.unique)
    })
  io.println("aco25::day7::puzzle0 " <> int.to_string(ret))
}

pub fn puzzle1() -> Nil {
  let input = parse_input()

  let assert [head, ..tail] = input

  let assert [init_pos] =
    head
    |> string.to_graphemes
    |> utils.index_of(fn(item1) { item1 == "S" })

  let pathes = dict.new()
  let pathes = pathes |> dict.insert(init_pos, 1)

  let pathes =
    tail
    |> list.fold(pathes, fn(acc, item) {
      let splitter_poses =
        item
        |> string.to_graphemes
        |> utils.index_of(fn(item1) { item1 == "^" })

      acc
      |> dict.fold(dict.new(), fn(acc, key, value) {
        case splitter_poses |> list.contains(key) {
          False -> {
            // update
            acc
            |> dict.upsert(key, fn(x) {
              case x {
                Some(x) -> x + value
                None -> value
              }
            })
          }
          True -> {
            // try left
            acc
            |> dict.upsert(key - 1, fn(x) {
              case x {
                Some(x) -> x + value
                None -> value
              }
            })
            |> dict.upsert(key + 1, fn(x) {
              case x {
                Some(x) -> x + value
                None -> value
              }
            })
          }
        }
      })
    })
  let ret =
    pathes
    |> dict.fold(0, fn(acc, _key, value) { acc + value })
  io.println("aco25::day7::puzzle1 " <> int.to_string(ret))
}
