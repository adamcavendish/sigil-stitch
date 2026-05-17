let transform: (Int) -> Int = {(x: Int) -> Int in return x * 2}
let sorted = items.sorted(by: {a, b in a.name < b.name})
