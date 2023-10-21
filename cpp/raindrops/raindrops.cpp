#include "raindrops.h"

#include <string>

namespace raindrops {
  std::string convert(int drops) {
    std::string result;

    if (drops % 3 == 0) {
      result += "Pling";
    }
    if (drops % 5 == 0) {
      result += "Plang";
    }
    if (drops % 7 == 0) {
      result += "Plong";
    }
    if (result.empty()) {
      result += std::to_string(drops);
    }

    return result;
  }
}  // namespace raindrops
