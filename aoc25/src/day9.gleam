import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile
import utils.{type Point, Point}

pub type Rectangle {
  Rectangle(top: Int, bottom: Int, left: Int, right: Int)
}

fn rectangle_from_vertices(p: Point, q: Point) -> Rectangle {
  let Point(x: qx, y: qy) = q
  let Point(x: px, y: py) = p

  let #(left, right) = case qx > px {
    True -> #(px, qx)
    False -> #(qx, px)
  }
  let #(bottom, top) = case qy > py {
    True -> #(py, qy)
    False -> #(qy, py)
  }

  Rectangle(top:, bottom:, left:, right:)
}

fn parse_input() -> List(Point) {
  let filename = "inputs/day9.txt"
  case simplifile.read(filename) {
    Ok(content) -> {
      content
      |> string.trim_end
      |> string.split(on: "\n")
      |> list.map(fn(line) {
        let assert [Ok(num0), Ok(num1)] =
          line |> string.split(on: ",") |> list.map(int.parse)
        Point(num0, num1)
      })
    }
    Error(_) -> panic as "invalid input"
  }
}

fn area(rectangle: Rectangle) {
  { rectangle.right - rectangle.left + 1 }
  * { rectangle.top - rectangle.bottom + 1 }
}

pub fn puzzle0() -> Nil {
  let input = parse_input()
  let assert Ok(rect) =
    input
    |> list.combination_pairs()
    |> list.map(fn(pair) { rectangle_from_vertices(pair.0, pair.1) })
    |> list.max(fn(one, other) { int.compare(area(one), area(other)) })

  let ret = area(rect)

  io.println("aco25::day9::puzzle0 " <> int.to_string(ret))
}

fn polygon_from_points(points: List(Point)) -> List(Rectangle) {
  let assert Ok(last) = list.last(points)
  let assert Ok(first) = list.first(points)

  [#(last, first), ..list.zip(points, list.drop(points, 1))]
  |> list.map(fn(pair) { rectangle_from_vertices(pair.0, pair.1) })
}

fn intersects(rectangle: Rectangle, with other: Rectangle) -> Bool {
  //                   ┌──────┐
  //           r.left  └──────┘ r.right
  //       s.left ├───────────────┤ s.right
  //
  //                            ┬  s.top
  //      r.top    ┌──────┐    │
  //      r.bottom └──────┘    │
  //                            ┴  s.bottom
  //

  // Great explanation someone linked on Reddit!
  // https://kishimotostudios.com/articles/aabb_collision/

  // Rectangle isn't to the right of the other
  rectangle.left < other.right
  // Rectangle isn't to the left of the other
  && rectangle.right > other.left
  // Rectangle isn't above the other
  && rectangle.bottom < other.top
  // Rectangle isn't below the other
  && rectangle.top > other.bottom
}

pub fn puzzle1() -> Nil {
  let input = parse_input()

  let polygon = polygon_from_points(input)

  let assert Ok(rect) =
    list.combination_pairs(input)
    |> list.map(fn(pair) { rectangle_from_vertices(pair.0, pair.1) })
    |> list.sort(fn(one, other) { int.compare(area(other), area(one)) })
    |> list.find(fn(rectange) { !list.any(polygon, intersects(_, rectange)) })

  let ret = area(rect)

  io.println("aco25::day9::puzzle1 " <> int.to_string(ret))
}
