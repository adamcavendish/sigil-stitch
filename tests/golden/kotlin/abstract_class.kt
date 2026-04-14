/**
 * Abstract shape.
 */
internal abstract class Shape {
    internal fun describe(): String {
        return this::class.simpleName ?: "Shape"
    }

    internal abstract fun area(): Double
}
