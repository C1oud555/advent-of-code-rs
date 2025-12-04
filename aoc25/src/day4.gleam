import gleam/int
import gleam/io
import gleam/list
import gleam/set
import gleam/string
import glearray.{type Array}
import simplifile

fn read_input_to_lines() -> List(String) {
  let filename = "inputs/day4.txt"
  case simplifile.read(filename) {
    Ok(content) -> content |> string.trim |> string.split(on: "\n")
    Error(_) -> []
  }
}

// padding dots
fn parse_input_to_array(input: List(String)) -> Array(Array(String)) {
  let ret =
    input
    |> list.map(fn(line) {
      [".", ..line |> string.to_graphemes |> list.append(["."])]
      |> glearray.from_list
    })
  let len = { ret |> list.length } + 2
  let pad = "." |> list.repeat(len) |> glearray.from_list

  [pad, ..ret |> list.append([pad])]
  |> glearray.from_list
}

fn get_item(arr: Array(Array(String)), row: Int, col: Int) -> String {
  let assert Ok(row) = arr |> glearray.get(row)
  let assert Ok(item) = row |> glearray.get(col)
  item
}

fn get_neightbors(arr: Array(Array(String)), row: Int, col: Int) -> List(String) {
  [
    get_item(arr, row - 1, col - 1),
    get_item(arr, row - 1, col),
    get_item(arr, row - 1, col + 1),
    get_item(arr, row, col - 1),
    // get_item(arr, row, col),
    get_item(arr, row, col + 1),
    get_item(arr, row + 1, col - 1),
    get_item(arr, row + 1, col),
    get_item(arr, row + 1, col + 1),
  ]
}

fn find_less_than_4(arr: Array(Array(String))) -> List(#(Bool, Int, Int)) {
  let len = arr |> glearray.length
  list.range(1, len - 2)
  |> list.map(fn(index_row) {
    list.range(1, len - 2)
    |> list.map(fn(index_col) {
      let neightbors = get_neightbors(arr, index_row, index_col)
      let len_of_rolls =
        neightbors
        |> list.filter(fn(item) { item == "@" })
        |> list.length
      let item = get_item(arr, index_row, index_col)
      case item {
        "@" if len_of_rolls < 4 -> #(True, index_row, index_col)
        _ -> #(False, index_row, index_col)
      }
    })
  })
  |> list.flatten
}

pub fn puzzle0() -> Nil {
  let input = read_input_to_lines()
  let array = parse_input_to_array(input)

  let ret =
    array
    |> find_less_than_4
    |> list.filter(fn(item) {
      let #(b, _, _) = item
      b
    })
    |> list.length

  io.println("aco25::day4::puzzle0 " <> int.to_string(ret))
}

fn solve1(input: Array(Array(String)), acc: Int) -> Int {
  let can_remove =
    input
    |> find_less_than_4
    |> list.filter(fn(item) {
      let #(b, _, _) = item
      b
    })
  let rlen =
    can_remove
    |> list.length

  let new_inpt = update_board(input, can_remove)
  case rlen > 0 {
    False -> acc
    True -> solve1(new_inpt, rlen + acc)
  }
}

fn update_board(
  value: Array(Array(String)),
  removel: List(#(Bool, Int, Int)),
) -> Array(Array(String)) {
  let coords_to_remove =
    removel
    |> list.map(fn(item) {
      let #(_, row, col) = item
      #(row, col)
    })
    |> set.from_list

  let list_of_lists = glearray.to_list(value) |> list.map(glearray.to_list)

  let new_list_of_lists = {
    let row_indices = list.range(0, list.length(list_of_lists) - 1)
    list.zip(list_of_lists, row_indices)
    |> list.map(fn(row_tuple) {
      let #(row_list, row_idx) = row_tuple
      let col_indices = list.range(0, list.length(row_list) - 1)
      list.zip(row_list, col_indices)
      |> list.map(fn(item_tuple) {
        let #(item, col_idx) = item_tuple
        case set.contains(coords_to_remove, #(row_idx, col_idx)) {
          True -> "."
          False -> item
        }
      })
    })
  }

  new_list_of_lists
  |> list.map(glearray.from_list)
  |> glearray.from_list
}

pub fn puzzle1() -> Nil {
  let input = read_input_to_lines()
  let array = parse_input_to_array(input)

  let ret = solve1(array, 0)

  io.println("aco25::day4::puzzle1 " <> int.to_string(ret))
}
