import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

fn read_input_to_lines() -> List(String) {
  let filename = "inputs/day3.txt"
  case simplifile.read(filename) {
    Ok(content) -> content |> string.trim |> string.split(on: "\n")
    Error(_) -> []
  }
}

pub fn puzzle0() -> Nil {
  let input = read_input_to_lines()
  let ret =
    input
    |> list.map(fn(line) {
      let line_nums =
        line
        |> string.to_graphemes
        |> list.filter_map(int.parse)
      let #(ho, lo) = line_nums |> list.split(list.length(line_nums) - 2)
      let ret = find_largest_n(lo, ho)
      let #(c_num, _) =
        ret
        |> list.fold_right(#(0, 1), fn(acc, item) {
          let #(acc_num, multi) = acc
          #(acc_num + multi * item, multi * 10)
        })
      c_num
    })
    |> list.fold(0, fn(acc, elem) { acc + elem })
  io.println("aco25::day3::puzzle0 " <> int.to_string(ret))
}

pub fn find_largest_n(init: List(Int), rest: List(Int)) -> List(Int) {
  let #(max, max_pos) =
    rest
    |> list.index_fold(#(0, 0), fn(acc, item, index) {
      let #(max_item, _) = acc
      case item > max_item {
        True -> #(item, index)
        False -> acc
      }
    })
  let #(_, rest_ho) = rest |> list.split(max_pos + 1)
  case init {
    [h, ..t] ->
      case h > max {
        True -> init
        False -> {
          [max, ..find_largest_n(t, rest_ho |> list.append([h]))]
        }
      }
    _ -> init
  }
}

pub fn puzzle1() -> Nil {
  let input = read_input_to_lines()
  let ret =
    input
    |> list.map(fn(line) {
      let line_nums =
        line
        |> string.to_graphemes
        |> list.filter_map(int.parse)
      let #(ho, lo) = line_nums |> list.split(list.length(line_nums) - 12)
      let ret = find_largest_n(lo, ho)
      let #(c_num, _) =
        ret
        |> list.fold_right(#(0, 1), fn(acc, item) {
          let #(acc_num, multi) = acc
          #(acc_num + multi * item, multi * 10)
        })
      c_num
    })
    |> list.fold(0, fn(acc, elem) { acc + elem })
  io.println("aco25::day3::puzzle1 " <> int.to_string(ret))
}
