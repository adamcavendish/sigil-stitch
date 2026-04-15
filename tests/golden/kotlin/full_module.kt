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

    internal override fun findById(id: String): User? {
        return users.firstOrNull { it.id == id }
    }

    internal override fun findAll(): List {
        return ArrayList(users)
    }
}
