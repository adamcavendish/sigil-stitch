public class UserService {
    public async Task<User> GetUserAsync(string id) {
        return await repo.GetByIdAsync(id);
    }
}
