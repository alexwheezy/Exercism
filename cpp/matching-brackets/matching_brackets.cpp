#include "matching_brackets.h"

#include <algorithm>
#include <sstream>
#include <stack>
#include <unordered_map>

namespace matching_brackets {
  bool check(const std::string &str) {
    if (str.empty()) {
      return true;
    }
    const std::unordered_map<char, char> brakets{
        {'}', '{'}, {')', '('}, {']', '['}};

    std::stack<char> match_brackets;

    std::istringstream ss{str};
    char c;

    while (ss >> c) {
      if (c == '{' || c == '(' || c == '[') {
        match_brackets.push(c);
      }
      if (!match_brackets.empty() && brakets.find(c) != brakets.end()) {
        if (match_brackets.top() == brakets.at(c)) {
          match_brackets.pop();
        } else {
          return false;
        }
      }
    }
    return match_brackets.empty();
  }
}  // namespace matching_brackets
