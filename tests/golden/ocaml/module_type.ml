(** Comparable interface. *)
module type COMPARABLE = sig
  val compare : t -> t -> int
end
