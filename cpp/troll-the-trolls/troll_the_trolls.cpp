namespace hellmath {

  enum class AccountStatus {
    troll,
    guest,
    user = 0x3,
    mod = 0x7,
  };

  enum class Action {
    read = 0x0,
    write = 0x1,
    remove = 0x2
  };

  constexpr bool display_post(AccountStatus status, AccountStatus viewer) {
    return (status == AccountStatus::troll) == (viewer == AccountStatus::troll);
  }

  constexpr bool permission_check(Action action, AccountStatus status) {
    return 1 << static_cast<int>(action) &
           (status == AccountStatus::troll ? 0x3 : static_cast<int>(status));
  }

  constexpr bool valid_player_combination(AccountStatus status,
                                          AccountStatus viewer) {
    return display_post(status, viewer) &
           !(status == AccountStatus::guest || viewer == AccountStatus::guest);
  }

  constexpr bool has_priority(AccountStatus status, AccountStatus viewer) {
    return status > viewer;
  }
}  // namespace hellmath
