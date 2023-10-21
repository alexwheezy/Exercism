#include "sieve.h"
#include <vector>
#include <cassert>

namespace sieve {
  std::vector<int> primes([[maybe_unused]] int n) {
    assert((n > 0 && "must be greater than zero"));

    std::vector<bool> composite(n + 1, false);
    std::vector<int> primes;
    primes.reserve(n / 2);

    for (int number = 2; number <= n; number++) {
      if (!composite[number]) {
        primes.emplace_back(number);
        for (int idx = number * number; idx <= n; idx += number)
          composite[idx] = true;
      }
    }
    return primes;
  }

}  // namespace sieve
