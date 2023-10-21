#include <algorithm>
#include <string>
#include <vector>

namespace election {

  // The election result struct is already created for you:

  struct ElectionResult {
    // Name of the candidate
    std::string name{};
    // Number of votes the candidate has
    int votes{};
  };

  int vote_count(const ElectionResult& election) {
    return election.votes;
  }

  void increment_vote_count(ElectionResult& election, int votes) {
    election.votes += votes;
  }

  ElectionResult& determine_result(std::vector<ElectionResult>& election) {
    auto it = std::max_element(
        election.begin(),
        election.end(),
        [](const ElectionResult& lhs, const ElectionResult& rhs) {
          return lhs.votes < rhs.votes;
        });
    it->name = "President " + it->name;
    return *it;
  }
}  // namespace election
