#[derive(Copy, Clone)]
pub enum Category {
    Ones = 1,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight = 15, // 1 + 2 + 3 + 4 + 5 dice side
    BigStraight = 20,    // 2 + 3 + 4 + 5 + 6 dice side
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(mut dice: Dice, category: Category) -> u8 {
    use Category::*;

    dice.sort_unstable();
    let any_combination = |item: Category| dice.iter().filter(|v| **v == (item as u8)).sum::<u8>();
    let all_combination = |slice: &[u8]| slice.iter().all(|v| v == &slice[0]);
    let sum_combination = || dice.iter().sum::<u8>();

    match category {
        Ones => any_combination(Ones),
        Twos => any_combination(Twos),
        Threes => any_combination(Threes),
        Fours => any_combination(Fours),
        Fives => any_combination(Fives),
        Sixes => any_combination(Sixes),
        LittleStraight if sum_combination() == LittleStraight as u8 => 30,
        BigStraight if sum_combination() == BigStraight as u8 => 30,
        #[rustfmt::skip]
        FourOfAKind => dice
            .windows(4)
            .filter(|arr| all_combination(&arr))
            .next().unwrap_or_default().iter()
            .sum::<u8>(),
        FullHouse => {
            if !all_combination(&dice)
                && (all_combination(&dice[0..2]) && all_combination(&dice[2..5])
                    || all_combination(&dice[0..3]) && all_combination(&dice[3..5]))
            {
                sum_combination()
            } else {
                0
            }
        }
        Yacht if all_combination(&dice) => 50,
        Choice => sum_combination(),
        _ => 0,
    }
}
