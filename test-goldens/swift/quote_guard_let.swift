func process(value: Int ?) {
    guard let unwrapped = value else {
        return
    }
    print(unwrapped)
}
