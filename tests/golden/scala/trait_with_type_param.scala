/**
 * Generic data repository.
 */
trait Repository[T] {
  def findById(id: String): Option[T]

  def save(entity: T)
}
