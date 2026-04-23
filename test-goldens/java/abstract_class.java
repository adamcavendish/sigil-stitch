/**
 * Abstract shape.
 */
public abstract class Shape {
    public String describe() {
        return this.getClass().getSimpleName();
    }

    public abstract double area();
}
