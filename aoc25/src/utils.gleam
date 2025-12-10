import gleam/list

pub fn list_sum(l: List(Int)) -> Int {
  l
  |> list.fold(0, fn(acc, item) { acc + item })
}

pub fn list_min(l: List(Int)) -> Int {
  let assert Ok(init) = l |> list.first
  l
  |> list.fold(init, fn(acc, item) {
    case item < acc {
      True -> item
      False -> acc
    }
  })
}

pub fn list_product(l: List(Int)) -> Int {
  l
  |> list.fold(1, fn(acc, item) { acc * item })
}

pub fn index_of(l: List(a), predicate: fn(a) -> Bool) -> List(Int) {
  l
  |> list.index_fold([], fn(acc, item, index) {
    case predicate(item) {
      True -> [index, ..acc]
      False -> acc
    }
  })
}

pub type Point {
  Point(x: Int, y: Int)
}
