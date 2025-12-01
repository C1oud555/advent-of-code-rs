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

pub fn parse_line(line: String) -> Result(#(String, Int), Nil) {
  case line {
    "L" <> num -> int.parse(num) |> result.map(fn(n) { #("L", n) })
    "R" <> num -> int.parse(num) |> result.map(fn(n) { #("R", n) })
    _ -> Error(Nil)
  }
}

pub fn puzzle0() -> Nil {
  let input = read_input_to_lines()
  let #(_, zero_cnt) =
    input
    |> list.filter_map(parse_line)
    |> list.fold(#(50, 0), fn(acc, parsed) {
      let #(st, zcnt) = acc
      let #(direction, num) = parsed
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

pub fn calculate_zero_landings(st: Int, num: Int, direction: String) -> Int {
  let initial_steps = num
  let current_st = st

  case direction {
    "R" -> {
      let steps_to_first_zero = case current_st {
        0 -> 100
        _ -> 100 - current_st
      }

      case initial_steps >= steps_to_first_zero {
        True -> {
          let new_landings = 1
          let remaining_steps = initial_steps - steps_to_first_zero
          new_landings + remaining_steps / 100
        }
        False -> 0
      }
    }
    "L" -> {
      let steps_to_first_zero = case current_st {
        0 -> 100
        _ -> current_st
      }

      case initial_steps >= steps_to_first_zero {
        True -> {
          let new_landings = 1
          let remaining_steps = initial_steps - steps_to_first_zero
          new_landings + remaining_steps / 100
        }
        False -> 0
      }
    }
    _ -> 0
  }
}

pub fn calculate_next_state_and_zeros(
  current_st: Int,
  total_num_steps: Int,
  direction: String,
) -> #(Int, Int) {
  let num_mod = total_num_steps % 100
  let new_st = case direction {
    "L" -> { current_st - num_mod + 100 } % 100
    "R" -> { current_st + num_mod } % 100
    _ -> current_st
    // Should not happen with valid input
  }

  let zero_landings =
    calculate_zero_landings(current_st, total_num_steps, direction)

  #(new_st, zero_landings)
}

pub fn puzzle1() -> Nil {
  let input = read_input_to_lines()
  let #(_, zero_cnt) =
    input
    |> list.filter_map(parse_line)
    |> list.fold(#(50, 0), fn(acc, parsed) {
      let #(st, zcnt) = acc
      let #(direction, num) = parsed
      let #(new_st, new_landings) =
        calculate_next_state_and_zeros(st, num, direction)
      #(new_st, zcnt + new_landings)
    })

  io.println("aco25::puzzle1 " <> zero_cnt |> int.to_string)
}
