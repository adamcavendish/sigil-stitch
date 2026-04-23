def compare[T : Ordering : Numeric](a: T, b: T): Int = {
  implicitly[Ordering[T]].compare(a, b)
}
