#include "collatz_conjecture.h"

#include <iostream>
namespace collatz_conjecture {
  unsigned steps(int n) {
    if (n <= 0) throw std::domain_error("");

    unsigned step = 0;
    while (n ^ 1) {
      n = ~n & 1 ? n >> 1 : n * 3 + 1;
      ++step;
    }
    return step;
  }
}  // namespace collatz_conjecture
