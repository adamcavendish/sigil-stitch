auto ptr = std::make_unique<Config>();
auto shared = std::make_shared<Node>(42);
std::weak_ptr<Node> weak = shared;
