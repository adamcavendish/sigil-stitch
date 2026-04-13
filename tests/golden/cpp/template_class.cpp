#pragma once

template<typename T>
/// A simple stack container.
class Stack {
private:
    std::vector<T> data_;

public:
    void push(const T& value) {
        data_.push_back(value);
    }

    bool empty() const {
        return data_.empty();
    }
};
