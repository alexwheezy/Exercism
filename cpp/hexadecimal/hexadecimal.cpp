#include "hexadecimal.h"

#include <cctype>
#include <cmath>
#include <cstdint>

namespace hexadecimal {
  int convert(const std::string& input) {
    if (input.empty()) {
      return 0;
    }

    int result = 0;
    const int magic_constant = 0x57;
    size_t size = input.size();

    for (size_t i = 0; i < size; ++i) {
      char c = input[i];
      int value = isdigit(c) ? c - '0' : c - magic_constant;
      if (value > 0xf) {
        return 0;
      }
      result += value * std::pow(16, size - i - 1);
    }
    return result;
  }

}  // namespace hexadecimal
