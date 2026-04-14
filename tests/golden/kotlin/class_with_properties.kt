/**
 * Service for managing users.
 */
class UserService {
    private var repo: UserRepository
    private val logger: Logger

    internal fun findUser(id: String): User {
        return repo.findById(id)
    }
}
