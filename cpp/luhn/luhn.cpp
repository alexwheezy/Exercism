#include "luhn.h"

#include <algorithm>
#include <cctype>
#include <cstdint>
#include <string>

namespace luhn {
  bool valid(const std::string& input) {
    std::string str{input};
    str.erase(std::remove_if(str.begin(), str.end(), isspace), str.end());

    if (str.size() <= 1) {
      return false;
    }

    uint16_t sum = 0;
    size_t idx = 0;
    for (auto it = str.rbegin(); it != str.rend(); ++it) {
      if (!isdigit(*it)) {
        return false;
      }
      uint8_t c = *it - '0';
      if (idx % 2 != 0) {
        c *= 2;
        c = c > 9 ? c - 9 : c;
      }
      sum += c;
      ++idx;
    }
    return !(sum % 10);
  }

}  // namespace luhn
