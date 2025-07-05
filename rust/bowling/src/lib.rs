const TOTAL_GAMES: usize = 10;
const BONUS_GAMES: usize = 2;

const MAX_THROW_PER_FRAME: usize = BONUS_GAMES;
const MAX_PINS: usize = TOTAL_GAMES;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Frame {
    #[default]
    Open,
    Spare,
    Strike,
}

#[derive(Default, Clone, Copy)]
pub struct Game {
    frame: Frame,
    throw: usize,
    scores: [usize; MAX_THROW_PER_FRAME],
    total_scores: usize,
}

impl Game {
    fn new() -> Self {
        Self {
            frame: Frame::Open,
            throw: 0,
            scores: [0; MAX_THROW_PER_FRAME],
            total_scores: 0,
        }
    }

    fn update_frame(&mut self, frame: Frame) {
        self.frame = frame;
    }
}

#[derive(Default)]
pub struct BowlingGame {
    //NOTE: Total number of games + possibly 2 bonus games
    games: [Game; TOTAL_GAMES + BONUS_GAMES],
    frame: usize,
    total_games: usize,
    total_throws: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            games: [Game::new(); TOTAL_GAMES + BONUS_GAMES],
            frame: 0,
            total_games: 0,
            total_throws: 0,
        }
    }

    pub fn roll(&mut self, pins: usize) -> Result<(), Error> {
        use Frame::*;

        let game = &mut self.games;
        let frame = &mut game[self.frame];
        let throw = frame.throw;

        frame.total_scores += pins;
        if frame.total_scores > MAX_PINS {
            return Err(Error::NotEnoughPinsLeft);
        }

        frame.scores[throw] = pins;
        frame.throw += 1;

        match (frame.total_scores, frame.throw) {
            (TOTAL_GAMES, 1) => {
                frame.update_frame(Strike);
                self.frame += 1;
                self.total_games += 1;
            }
            (TOTAL_GAMES, MAX_THROW_PER_FRAME) => {
                frame.update_frame(Spare);
                self.frame += 1;
                self.total_games += 1;
            }
            (_, MAX_THROW_PER_FRAME) => {
                self.frame += 1;
                self.total_games += 1;
            }
            _ => (),
        }
        self.total_throws += 1;
        // Index 9 is the last frame
        match (self.total_games, self.total_throws, &game[9].frame) {
            (TOTAL_GAMES, 21, Open) | (_, 22, _) => return Err(Error::GameComplete),
            _ => (),
        }

        Ok(())
    }

    pub fn score(&self) -> Option<usize> {
        use Frame::*;

        if self.total_games < TOTAL_GAMES {
            return None;
        }

        let (last_frame, bonus_frame_one, bonus_frame_two) = (
            self.games[9].frame,
            self.games[10].frame,
            self.games[11].frame,
        );

        //TODO: We can probably remove the if statement?
        if let (10, _, Strike, _, _) | (11, _, Strike, Strike, _) | (10, 20, Spare, _, _) = (
            self.frame,
            self.total_throws,
            last_frame,
            bonus_frame_one,
            bonus_frame_two,
        ) {
            return None;
        }

        let mut score: usize = 0;
        // We take the first 10 frames
        for (idx, game) in self.games.iter().take(10).enumerate() {
            match game.frame {
                Strike => match (&self.games[idx + 1].frame, &self.games[idx + 2].frame) {
                    // Maximum points for 3 strikes: 10 + 10 + 10 = 30
                    (Strike, Strike) => score += MAX_PINS * 3,
                    // Maximum points for 2 strikes: 10 + 10 = 20
                    (Strike, Spare | Open) => score += MAX_PINS * 2 + self.games[idx + 2].scores[0],
                    // Maximum points for 1 strike: 10
                    (Spare | Open, _) => score += MAX_PINS + self.games[idx + 1].total_scores,
                },
                Spare => score += game.total_scores + self.games[idx + 1].scores[0],
                Open => score += game.total_scores,
            }
        }
        Some(score)
    }
}
