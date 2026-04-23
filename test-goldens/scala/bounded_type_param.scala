def max[T <: Comparable[T]](a: T, b: T): T = {
  if (a.compareTo(b) >= 0) a else b
}
