/**
 * A sorted container with bounded type parameter.
 */
public class SortedContainer<T extends Comparable & Serializable> {
    private List<T> items;

    public void add(T item) {
        this.items.add(item);
    }
}
