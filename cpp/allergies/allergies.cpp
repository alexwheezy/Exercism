#include "allergies.h"

namespace allergies {
  bool allergy_test::is_allergic_to(const std::string &input) {
    return allergy_test::allergies_category.count(input);
  }
  const allergy_test::Allergy allergy_test::get_allergies() const {
    return allergy_test::allergies_category;
  }

}  // namespace allergies
