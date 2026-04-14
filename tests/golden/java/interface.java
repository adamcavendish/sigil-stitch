/**
 * Generic data repository.
 */
public interface Repository<T> {
    T findById(String id);

    void save(T entity);

    void delete(String id);
}
