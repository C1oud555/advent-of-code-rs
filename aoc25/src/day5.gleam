import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

type Range {
  Range(start: Int, end: Int)
}

fn contains(range: Range, val: Int) -> Bool {
  range.start <= val && val <= range.end
}

fn parse_input() -> #(List(Range), List(Int)) {
  let filename = "inputs/day5.txt"
  case simplifile.read(filename) {
    Ok(content) -> {
      let assert Ok(#(ranges, ingredients)) =
        content |> string.trim |> string.split_once(on: "\n\n")

      let ranges =
        ranges
        |> string.split("\n")
        |> list.map(fn(line) {
          let assert Ok(#(start, end)) = line |> string.split_once("-")
          let assert Ok(start) = start |> int.parse
          let assert Ok(end) = end |> int.parse
          Range(start, end)
        })

      let ingredients =
        ingredients
        |> string.split("\n")
        |> list.map(fn(line) {
          let assert Ok(ingre) = line |> int.parse
          ingre
        })

      #(ranges, ingredients)
    }
    Error(_) -> panic as "not valid input"
  }
}

pub fn puzzle0() -> Nil {
  let #(ranges, val) = parse_input()
  let ret =
    val
    |> list.filter(fn(item) {
      ranges |> list.any(fn(range) { range |> contains(item) })
    })
    |> list.length

  io.println("aco25::day5::puzzle0 " <> int.to_string(ret))
}

fn merge_range(ranges: List(Range)) -> List(Range) {
  ranges
  |> list.sort(fn(a, b) {
    let Range(sa, _) = a
    let Range(sb, _) = b
    sa |> int.compare(sb)
  })
  |> list.fold([], fn(acc, item) {
    let Range(start, end) = item

    case acc {
      [head, ..tail] -> {
        let Range(last_start, last_end) = head
        case last_end >= start, last_end >= end {
          True, True -> acc
          True, False -> [Range(last_start, end), ..tail]
          False, _ -> [item, ..acc]
        }
      }
      _ -> [item]
    }
  })
}

pub fn puzzle1() -> Nil {
  let #(ranges, _) = parse_input()

  let ranges = merge_range(ranges)

  let ret =
    ranges
    |> list.fold(0, fn(acc, range) {
      echo range
      acc + range.end - range.start + 1
    })

  io.println("aco25::day5::puzzle1 " <> int.to_string(ret))
}
