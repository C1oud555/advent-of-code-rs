import gleam/list

pub fn list_sum(l: List(Int)) -> Int {
  l
  |> list.fold(0, fn(acc, item) { acc + item })
}

pub fn list_product(l: List(Int)) -> Int {
  l
  |> list.fold(1, fn(acc, item) { acc * item })
}
