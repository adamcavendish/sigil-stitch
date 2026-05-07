/// Generic data repository.
public interface IRepository<T> {
    internal T FindById(string id);

    internal void Save(T entity);
}
