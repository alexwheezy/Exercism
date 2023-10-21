#include "trinary.h"

#include <cmath>

namespace trinary {
  unsigned to_decimal(const std::string& input) {
    unsigned result = 0;
    if (input.empty()) {
      return result;
    }

    for (size_t i = 0; i < input.size(); ++i) {
      if (input[i] - '0' <= 2) {
        result += input[i] - '0' * std::pow(3, input.size() - 1 - i);
      }
    }
    return result;
  }

}  // namespace trinary
