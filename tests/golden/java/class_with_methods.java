/**
 * Service for managing users.
 */
public class UserService {
    private UserRepository repo;
    private final Logger logger;

    public UserService(UserRepository repo, Logger logger) {
        this.repo = repo;
        this.logger = logger;
    }

    public User findUser(String id) {
        return this.repo.findById(id);
    }
}
