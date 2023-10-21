#include "bob.h"

#include <algorithm>
#include <iostream>

namespace bob {
  std::string hey(const std::string& str) {
    std::string input(str);
    input.erase(std::remove_if(input.begin(),
                               input.end(),
                               [](unsigned char c) {
                                 return std::isspace(c) ||
                                        (std::ispunct(c) && c != '?');
                               }),
                input.end());

    bool questions = input.find_last_of('?') == input.size() - 1;
    bool screaming =
        std::all_of(input.begin(),
                    input.end(),
                    [](unsigned char c) { return std::isupper(c); });

    if (input.empty()) {
      return "Fine. Be that way!";
    } else if (questions) {
      if (screaming) {
        return "Calm down, I know what I'm doing!";
      }
      return "Sure.";
    } else if (screaming) {
      return "Whoa, chill out!";
    }
    return "Whatever.";
  }

}  // namespace bob
