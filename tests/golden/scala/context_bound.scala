def sortedPair[T : Ordering](a: T, b: T): Tuple2[T, T] = {
  implicitly[Ordering[T]].compare(a, b)
}
