import com.example.model.User

internal suspend fun fetchUser(id: String): User {
    return api.fetchUser(id)
}
