import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

fn read_input_to_lines() -> List(String) {
  let filename = "inputs/day2.txt"
  case simplifile.read(filename) {
    Ok(content) -> content |> string.trim |> string.split(on: ",")
    Error(_) -> []
  }
}

pub fn parse_line(line: String) -> Result(#(Int, Int), Nil) {
  let nums = string.split(line, on: "-")
  let ints = nums |> list.filter_map(int.parse)
  case ints {
    [h, t] -> Ok(#(h, t))
    _ -> Error(Nil)
  }
}

pub fn puzzle0() -> Nil {
  let input = read_input_to_lines()
  let ret =
    input
    |> list.filter_map(parse_line)
    |> list.fold(0, fn(acc, parsed) {
      let #(start, end) = parsed
      let ret =
        list.range(start, end)
        |> list.fold([], fn(acc, elem) {
          let s_str = int.to_string(elem)
          let s_len = string.length(s_str)
          let he = string.slice(s_str, 0, s_len / 2)
          let le = string.slice(s_str, s_len / 2, s_len - s_len / 2)
          case he == le {
            True -> [elem, ..acc]
            False -> acc
          }
        })
        |> list.fold(0, fn(acc, elem) { acc + elem })
      ret + acc
    })

  io.println("aco25::day2::puzzle0 " <> ret |> int.to_string)
}

pub fn puzzle1() -> Nil {
  let input = read_input_to_lines()

  let ret =
    input
    |> list.filter_map(parse_line)
    |> list.fold(0, fn(acc, parsed) {
      let #(start, end) = parsed
      let ret =
        list.range(start, end)
        |> list.fold([], fn(acc0, elem0) {
          let s_str = int.to_string(elem0)
          let s_len = string.length(s_str)
          let repeat_list =
            list.range(1, s_len / 2)
            |> list.fold([], fn(acc1, elem1) {
              let he = string.slice(s_str, 0, elem1)
              let repeat_times = s_len / elem1
              let repeat_str = he |> string.repeat(repeat_times)
              case repeat_str == s_str && s_len >= 2 {
                True -> {
                  [elem0, ..acc1]
                }
                False -> acc1
              }
            })
          let unique_repeat_list = repeat_list |> list.unique
          list.append(acc0, unique_repeat_list)
        })
        |> list.fold(0, fn(acc, elem) { acc + elem })
      ret + acc
    })

  io.println("aco25::day2::puzzle0 " <> ret |> int.to_string)
}
