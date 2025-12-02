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

// Helper function for integer power, as it's not in the standard library.
fn int_power(base: Int, exponent: Int) -> Int {
  case exponent {
    0 -> 1
    1 -> base
    _ -> base * int_power(base, exponent - 1)
  }
}

// Optimized check for puzzle0. Works with integers directly, avoiding string conversions in the check.
fn is_split_equal(n: Int) -> Bool {
  let s = int.to_string(n)
  let len = string.length(s)

  case len % 2 != 0 {
    True ->
      // Odd number of digits can't be split into equal halves.
      False
    False -> {
      let k = len / 2
      let divisor = int_power(10, k)
      let first_half = n / divisor
      let second_half = n % divisor
      first_half == second_half
    }
  }
}

pub fn puzzle0() -> Nil {
  let input = read_input_to_lines()
  let ret =
    input
    |> list.filter_map(parse_line)
    |> list.fold(0, fn(acc, parsed) {
      let #(start, end) = parsed
      let range_sum =
        list.range(start, end)
        |> list.filter(is_split_equal)
        |> list.fold(0, fn(acc, elem) { acc + elem })
      range_sum + acc
    })

  io.println("aco25::day2::puzzle0_optimized " <> int.to_string(ret))
}

fn is_repeat_number(n: Int) -> Bool {
  let s_str = int.to_string(n)
  let s_len = string.length(s_str)
  case s_len < 2 {
    True -> False
    False ->
      list.range(1, s_len / 2 + 1)
      |> list.any(fn(prefix_len) {
        case s_len % prefix_len == 0 {
          True -> {
            let prefix = string.slice(s_str, 0, prefix_len)
            let repeat_times = s_len / prefix_len
            prefix |> string.repeat(repeat_times) == s_str
          }
          False -> False
        }
      })
  }
}

pub fn puzzle1() -> Nil {
  let input = read_input_to_lines()

  let ret =
    input
    |> list.filter_map(parse_line)
    |> list.fold(0, fn(acc, parsed) {
      let #(start, end) = parsed
      let range_sum =
        list.range(start, end)
        |> list.filter(is_repeat_number)
        |> list.fold(0, fn(acc, elem) { acc + elem })
      range_sum + acc
    })

  io.println("aco25::day2::puzzle1_optimized " <> int.to_string(ret))
}
