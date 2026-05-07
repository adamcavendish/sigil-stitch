public class SortedList<T : IComparable> {
    private List<T> items;

    public void Add(T item) {
        items.Add(item);
        items.Sort();
    }
}
