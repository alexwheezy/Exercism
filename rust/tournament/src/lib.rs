use int_enum::IntEnum;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Game {
    Win = 3,
    Draw = 1,
    Loss = 0,
}

#[derive(Debug, Clone)]
pub struct Stats {
    mp: u16,
    win: u16,
    draw: u16,
    loss: u16,
    points: u16,
}

#[derive(Debug, Clone)]
pub struct Team {
    name: String,
    stats: Stats,
}

impl Team {
    fn new(name: String) -> Self {
        Team {
            name,
            stats: Stats {
                mp: 0,
                win: 0,
                draw: 0,
                loss: 0,
                points: 0,
            },
        }
    }

    fn update_team_stats(&mut self, game: Game) {
        self.stats.mp += 1;
        match game {
            Game::Win => {
                self.stats.win += 1;
                self.stats.points += Game::Win.int_value() as u16
            }
            Game::Draw => {
                self.stats.draw += 1;
                self.stats.points += Game::Draw.int_value() as u16
            }
            Game::Loss => self.stats.loss += 1,
        }
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n{:<}{2:>1$}{3:>3} |{4:>3} |{5:>3} |{6:>3} |{7:>3}",
            self.name,
            32 - self.name.len(),
            "|",
            self.stats.mp,
            self.stats.win,
            self.stats.draw,
            self.stats.loss,
            self.stats.points,
        )
    }
}

pub fn update_map<'a>(map: &mut HashMap<&'a str, Team>, name: &'a str, result: Game) {
    match map.get_mut(name) {
        Some(key) => {
            key.update_team_stats(result);
        }
        None => {
            map.insert(name, Team::new(name.to_string()));
            map.get_mut(name).unwrap().update_team_stats(result);
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table_result = format!("Team                           | MP |  W |  D |  L |  P");
    if match_results.is_empty(){
        return table_result.to_string();
    }

    let mut res_map: HashMap<&str, Team> = HashMap::new();
    let strings = match_results.trim().split('\n').collect::<Vec<_>>();

    for &string in strings.iter() {
        let data = string.trim().split(';').collect::<Vec<_>>();
        match data[2] {
            "win" => {
                update_map(&mut res_map, &data[0], Game::Win);
                update_map(&mut res_map, &data[1], Game::Loss);
            }
            "loss" => {
                update_map(&mut res_map, &data[1], Game::Win);
                update_map(&mut res_map, &data[0], Game::Loss);
            }
            _ => {
                update_map(&mut res_map, &data[1], Game::Draw);
                update_map(&mut res_map, &data[0], Game::Draw);
            }
        }
    }

    let mut teams = res_map.values().cloned().collect::<Vec<_>>();
    teams.sort_by(|team1, team2| {
        let points = team1.stats.points.cmp(&team2.stats.points).reverse();
        let name = team1.name.cmp(&team2.name);
        if points.is_eq() {
            name
        } else {
            points
        }
    });


    teams
        .iter()
        .for_each(|team| table_result.push_str(format!("{}", team).as_str()));
    table_result
}