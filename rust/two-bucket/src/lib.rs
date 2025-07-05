#[derive(PartialEq, Copy, Clone, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

enum BucketState {
    Empty,
    Partial,
    Full,
}

struct BucketData {
    capacity: u8,
    fill: u8,
    state: BucketState,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    use BucketState::*;

    if capacity_1.max(capacity_2) < goal || capacity_1 == capacity_2 {
        return None;
    }

    let (mut bucket1, mut bucket2) = (
        BucketData {
            capacity: capacity_1,
            fill: 0,
            state: Empty,
        },
        BucketData {
            capacity: capacity_2,
            fill: 0,
            state: Empty,
        },
    );

    let mut moves: u8 = 0;

    if start_bucket == &Bucket::One {
        while bucket1.fill != goal && bucket2.fill != goal {
            match (&bucket1.state, &bucket2.state) {
                (Full, Partial) => {
                    let sum = bucket1.fill + bucket2.fill;

                    bucket1.fill = sum.max(bucket2.capacity) - bucket2.capacity;
                    bucket1.state = if bucket1.fill == 0 { Empty } else { Partial };

                    bucket2.fill = bucket2.capacity.min(sum);
                    bucket2.state = if bucket2.fill == bucket2.capacity {
                        Full
                    } else {
                        Partial
                    };
                }
                (Partial, Full) => {
                    bucket2.state = Empty;
                }
                (Empty, Partial) => {
                    bucket1.fill = bucket1.capacity;
                    bucket1.state = Full;
                }
                (Partial, Empty) => {
                    bucket2.fill = bucket1.fill;
                    bucket2.state = Partial;
                    bucket1.state = Empty;
                }
                (Full, Full) => {
                    bucket2.fill = bucket1.fill;
                    bucket2.state = Partial;
                }
                (Full, Empty) => {
                    bucket2.fill = bucket2.capacity;
                    bucket2.state = Full;
                    bucket1.state = Full;
                }
                (Empty, Empty) => {
                    bucket1.fill = bucket1.capacity;
                    bucket1.state = Full;
                }
                (Empty, Full) => return None,
                _ => (),
            }
            moves += 1;
        }
    } else {
        while bucket1.fill != goal && bucket2.fill != goal {
            match (&bucket1.state, &bucket2.state) {
                (Empty, Full) => {
                    bucket2.fill -= bucket1.capacity;
                    bucket1.state = Full;
                    bucket2.state = Partial;
                }
                (Full, Partial) => {
                    bucket1.state = Empty;
                }
                (Empty, Partial) => {
                    bucket1.fill = bucket2.fill.min(bucket1.capacity);
                    bucket1.state = if bucket1.fill == bucket1.capacity {
                        Full
                    } else {
                        Partial
                    };
                    bucket2.fill = if bucket2.fill < bucket1.capacity {
                        bucket2.state = Empty;
                        0
                    } else {
                        bucket2.state = Partial;
                        bucket2.fill - bucket1.capacity
                    };
                }
                (Partial, Empty) => {
                    bucket2.fill = bucket2.capacity;
                    bucket2.state = Full;
                }
                (Partial, Full) => {
                    bucket2.fill -= bucket1.capacity - bucket1.fill;
                    bucket1.state = Full;
                    bucket2.state = Partial;
                }
                (Empty, Empty) => {
                    bucket2.fill = bucket2.capacity;
                    bucket2.state = Full;
                }
                (Full, Empty) => return None,
                _ => (),
            }
            moves += 1;
        }
    }

    let (goal_bucket, other_bucket) = if bucket1.fill == goal {
        (Bucket::One, bucket2.fill)
    } else {
        (Bucket::Two, bucket1.fill)
    };

    Some(BucketStats {
        moves,
        goal_bucket,
        other_bucket,
    })
}
