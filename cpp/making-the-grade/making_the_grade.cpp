#include <algorithm>
#include <array>
#include <iterator>
#include <sstream>
#include <string>
#include <vector>

// Round down all provided student scores.
std::vector<int> round_down_scores(std::vector<double> student_scores) {
  std::vector<int> scores(student_scores.size());
  std::transform(
      student_scores.begin(), student_scores.end(), scores.begin(), [](int v) {
        return static_cast<int>(v);
      });
  return scores;
}

// Count the number of failing students out of the group provided.
int count_failed_students(std::vector<int> student_scores) {
  auto count = std::count_if(student_scores.begin(),
                             student_scores.end(),
                             [](int v) { return v <= 40; });

  return count;
}

// Determine how many of the provided student scores were 'the best' based on
// the provided threshold.
std::vector<int> above_threshold(std::vector<int> student_scores,
                                 int threshold) {
  auto it = std::remove_if(student_scores.begin(),
                           student_scores.end(),
                           [=](int v) { return v < threshold; });
  student_scores.erase(it, student_scores.end());
  return std::vector<int>(student_scores.begin(), student_scores.end());
}

// Create a list of grade thresholds based on the provided highest grade.
std::array<int, 4> letter_grades(int highest_score) {
  int delta = highest_score / 8;
  int a = highest_score - delta;
  int b = a - delta - 1;
  int c = b - delta - 1;
  int d = c - delta - 1;
  return std::array<int, 4>{d, c, b, a};
}

// Organize the student's rank, name, and grade information in ascending order.
std::vector<std::string> student_ranking(
    std::vector<int> student_scores,
    std::vector<std::string> student_names) {
  std::vector<std::string> ranking(student_scores.size());
  for (size_t i = 0; i < student_scores.size(); ++i) {
    std::stringstream ss;
    ss << i + 1 << ". " << student_names[i] << ": " << student_scores[i];
    ranking[i] = ss.str();
  }
  return ranking;
}

// Create a string that contains the name of the first student to make a perfect
// score on the exam.
std::string perfect_score(std::vector<int> student_scores,
                          std::vector<std::string> student_names) {
  auto it = std::find(student_scores.begin(), student_scores.end(), 100);
  return it != student_scores.end()
             ? student_names[std::distance(student_scores.begin(), it)]
             : "";
}
