class Animal {
public:
    virtual void speak() const = 0;
};

class Dog : public Animal, public Serializable {
public:
    void speak() const override {
        // bark
    }
};
