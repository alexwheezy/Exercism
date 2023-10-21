#include "high_scores.h"

#include <algorithm>

namespace arcade {

  std::vector<int> HighScores::list_scores() {
    // TODO: Return all scores for this session.
    return HighScores::scores;
  }

  int HighScores::latest_score() {
    // TODO: Return the latest score for this session.
    return HighScores::scores.back();
  }

  int HighScores::personal_best() {
    // TODO: Return the highest score for this session.
    return *std::max_element(HighScores::scores.begin(),
                             HighScores::scores.end());
  }

  std::vector<int> HighScores::top_three() {
    // TODO: Return the top 3 scores for this session in descending order.
    std::vector<int> result(HighScores::scores);
    std::stable_sort(result.begin(), result.end(), std::greater<int>());
    result.resize(std::min(result.size(), static_cast<size_t>(3)));

    return result;
  }

}  // namespace arcade
