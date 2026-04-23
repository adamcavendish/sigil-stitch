(** Transform a list using a mapping function. *)
let transform (f : 'a -> 'b) (xs : 'a list) : 'b list =
  List.map f xs
