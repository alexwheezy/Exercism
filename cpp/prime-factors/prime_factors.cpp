#include "prime_factors.h"
#include <vector>
#include <cassert>

namespace prime_factors {
  std::vector<int> of(int n) {
    assert((n > 0 && "must be greater than zero"));

    std::vector<int> result;
    int d = n;
    int i = 2;

    while (d != 1) {
      if (d % i == 0) {
        result.push_back(i);
        d /= i;
      } else {
        i += 1;
      }
    }
    return result;
  }
}  // namespace prime_factors
