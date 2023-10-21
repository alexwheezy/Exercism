#include "darts.h"

#include <cmath>

namespace darts {
  size_t score(double x, double y) {
    double radius = std::sqrt(std::pow(0.0 - x, 2) + std::pow(0.0 - y, 2));
    size_t result = 0;
    if (radius <= 1) {
      result += 10;
    } else if (radius > 1 && radius <= 5) {
      result += 5;
    } else if (radius > 5 && radius <= 10) {
      result += 1;
    }
    return result;
  }

}  // namespace darts
