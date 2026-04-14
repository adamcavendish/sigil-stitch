/**
 * Generic data repository.
 */
internal interface Repository<T> {
    internal fun findById(id: String): T?

    internal fun save(entity: T)

    internal fun delete(id: String)
}
