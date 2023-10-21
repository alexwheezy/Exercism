#include <algorithm>
namespace targets {
  class Alien {
  public:
    Alien(int x, int y): x_coordinate(x), y_coordinate(y) {}

    int get_health() { return health; }
    bool hit() {
      --health;
      return true;
    }
    bool is_alive() const { return health > 0; }
    bool teleport(int new_x, int new_y) {
      x_coordinate = new_x;
      y_coordinate = new_y;
      return true;
    }
    bool collision_detection(const Alien& other) {
      return x_coordinate == other.x_coordinate &&
             y_coordinate == other.y_coordinate;
    }

    int x_coordinate;
    int y_coordinate;

  private:
    int health{3};
  };

}  // namespace targets
