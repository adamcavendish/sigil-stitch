/**
 * Abstract shape.
 */
abstract class Shape {
  def describe(): String = {
    getClass.getSimpleName
  }

  abstract def area(): Double
}
