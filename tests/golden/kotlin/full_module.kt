import kotlin.collections.ArrayList
import kotlin.collections.List
import kotlin.collections.MutableList

internal interface UserRepository {
    internal fun findById(id: String): User?

    internal fun findAll(): List
}

/**
 * In-memory implementation of UserRepository.
 */
internal class InMemoryUserRepository : UserRepository {
    private val users: MutableList

    override
    internal fun findById(id: String): User? {
        return users.firstOrNull { it.id == id }
    }

    override
    internal fun findAll(): List {
        return ArrayList(users)
    }
}
