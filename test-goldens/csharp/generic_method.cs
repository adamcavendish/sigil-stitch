public class Utils {
    public static T Max<T : IComparable<T>>(T a, T b) {
        return a.CompareTo(b) > 0 ? a : b;
    }
}
