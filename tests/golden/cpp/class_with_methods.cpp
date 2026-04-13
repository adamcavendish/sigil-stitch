#pragma once

/// A simple counter class.
class Counter {
private:
    int count_;

public:
    Counter() {
        count_ = 0;
    }

    void increment() {
        ++count_;
    }

    int get_count() const {
        return count_;
    }
};
