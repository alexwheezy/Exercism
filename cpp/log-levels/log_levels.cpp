#include <algorithm>
#include <cctype>
#include <iostream>
#include <string>

namespace log_line {

  std::string message(const std::string& log) {
    size_t pos = log.find(' ');
    if (pos == log.npos) {
      return "";
    }
    return log.substr(pos + 1, log.size() - pos);
  }

  std::string log_level(const std::string& log) {
    size_t start = log.find('[');
    size_t end = log.find(']');
    if (start == log.npos || end == log.npos) {
      return "";
    }
    auto substr = log.substr(start + 1, end - 1);
    std::transform(
        substr.begin(), substr.end(), substr.begin(), [](unsigned char c) {
          return std::toupper(c);
        });
    return substr;
  }

  std::string reformat(const std::string& log) {
    auto msg = message(log);
    auto level = log_level(log);
    return msg + " (" + level + ")";
  }

}  // namespace log_line
