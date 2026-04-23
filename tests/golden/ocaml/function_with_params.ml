let transform (f : 'a -> 'b) (xs : 'a list) : 'b list =
  List.map f xs
