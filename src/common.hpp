#pragma once
#include <string>
#include <memory>
#include <vector>
#include <optional>
#include <stdexcept>
#include <iostream>
#include <variant>
#include <list>

#include "errors.hpp"

// ValueData type alias
using ValueData = std::variant<std::monostate, int, float, std::string, bool, std::list<int>, std::list<float>, std::list<std::string>, std::list<bool>>;