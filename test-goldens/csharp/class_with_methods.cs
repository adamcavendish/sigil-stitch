/// Service for managing users.
public class UserService {
    private UserRepository repo;
    private readonly ILogger logger;

    public UserService(UserRepository repo, ILogger logger) {
        this.repo = repo;
        this.logger = logger;
    }

    public User FindUser(string id) {
        return this.repo.FindById(id);
    }
}
