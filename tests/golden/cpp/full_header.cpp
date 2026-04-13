#pragma once

#include <iostream>
#include <string>

#include "base.hpp"

// Uses Base

/// Application logger.
class Logger : public Base {
private:
    std::string name_;

public:
    Logger(const std::string& name) {
        name_ = name;
    }

    void log(const char* msg) {
        std::cout << name_ << ": " << std::string(msg) << std::endl;
    }

    const std::string& name() const {
        return name_;
    }
};
