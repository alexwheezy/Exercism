#if !defined(ALLERGIES_H)
#define ALLERGIES_H

#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>

namespace allergies {
  struct allergy_test {
    using Allergy = std::unordered_set<std::string>;

    allergy_test(unsigned score): score_(score) {
      for (auto category: allergy_type) {
        if ((score_ & category.first) == category.first)
          allergies_category.emplace(category.second);
      }
    }
    bool is_allergic_to(const std::string&);
    const Allergy get_allergies() const;

  private:
    const std::unordered_map<unsigned, std::string> allergy_type{
        {1, "eggs"},
        {2, "peanuts"},
        {4, "shellfish"},
        {8, "strawberries"},
        {16, "tomatoes"},
        {32, "chocolate"},
        {64, "pollen"},
        {128, "cats"}};
    Allergy allergies_category;
    unsigned score_;
  };

}  // namespace allergies

#endif  // ALLERGIES_H
