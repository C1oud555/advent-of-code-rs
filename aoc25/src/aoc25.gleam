import argv
import clip.{type Command}
import clip/flag.{type Flag}
import clip/help
import clip/opt.{type Opt}
import day10
import day3
import day4
import day5
import day6
import day7
import day8
import day9
import gleam/io
import gleam/list

import day1
import day2

type Arg {
  Arg(day: Int, all: Bool)
}

fn day_opt() -> Opt(Int) {
  opt.new("day") |> opt.short("d") |> opt.int |> opt.help("day number")
}

fn all_opt() -> Flag {
  flag.new("all") |> flag.short("a") |> flag.help("run all test")
}

fn command() -> Command(Arg) {
  clip.command({
    use day <- clip.parameter
    use all <- clip.parameter
    Arg(day - 1, all)
  })
  |> clip.opt(day_opt())
  |> clip.flag(all_opt())
}

pub fn main() {
  let args =
    command()
    |> clip.help(help.simple("aoc25", "run any aoc25 solution"))
    |> clip.run(argv.load().arguments)

  case args {
    Error(e) -> {
      io.println_error(e)
    }
    Ok(args) -> {
      run_test(args)
    }
  }
}

fn run_test(args: Arg) {
  let puzzles = [
    #(day1.puzzle0, day1.puzzle1),
    #(day2.puzzle0, day2.puzzle1),
    #(day3.puzzle0, day3.puzzle1),
    #(day4.puzzle0, day4.puzzle1),
    #(day5.puzzle0, day5.puzzle1),
    #(day6.puzzle0, day6.puzzle1),
    #(day7.puzzle0, day7.puzzle1),
    #(day8.puzzle0, day8.puzzle1),
    #(day9.puzzle0, day9.puzzle1),
    #(day10.puzzle0, day10.puzzle1),
  ]
  case args.all {
    True ->
      puzzles
      |> list.each(fn(funs) {
        let #(p0, p1) = funs
        p0()
        p1()
      })
    False -> {
      let funs =
        puzzles
        |> list.drop(args.day)
        |> list.first
      case funs {
        Ok(funs) -> {
          let #(p0, p1) = funs
          p0()
          p1()
        }
        Error(_) -> Nil
      }
    }
  }
}
