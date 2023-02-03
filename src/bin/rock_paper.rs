#![allow(dead_code, unused)]

use std::str::FromStr;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pick {
    Rock,
    Paper,
    Scissor,
}

use Pick::*;

impl FromStr for Pick {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissor),
            _ => Err("Unrecognized pick".into()),
        }
    }
}

#[derive(Debug)]
struct Turn {
    opponent_pick: Pick,
    self_pick: Pick,
}

enum TurnResult {
    Draw,
    Winner,
    Loser,
}

impl FromStr for TurnResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<TurnResult> {
        match s {
            "X" => Ok(TurnResult::Loser),
            "Y" => Ok(TurnResult::Draw),
            "Z" => Ok(TurnResult::Winner),
            _ => unreachable!(),
        }
    }
}

impl Turn {
    fn score(&self, score_cfg: &ScoreConfig) -> u32 {
        let turn_result_score = match self.turn_result() {
            TurnResult::Draw => score_cfg.draw,
            TurnResult::Loser => score_cfg.loser,
            TurnResult::Winner => score_cfg.winner,
        };

        let pick_score = match self.self_pick {
            Rock => score_cfg.rock,
            Paper => score_cfg.paper,
            Scissor => score_cfg.scissor,
        };

        turn_result_score + pick_score
    }

    fn turn_result(&self) -> TurnResult {
        match self.self_pick {
            Rock => match self.opponent_pick {
                Rock => TurnResult::Draw,
                Paper => TurnResult::Loser,
                Scissor => TurnResult::Winner,
            },
            Paper => match self.opponent_pick {
                Rock => TurnResult::Winner,
                Paper => TurnResult::Draw,
                Scissor => TurnResult::Loser,
            },
            Scissor => match self.opponent_pick {
                Rock => TurnResult::Loser,
                Paper => TurnResult::Winner,
                Scissor => TurnResult::Draw,
            },
        }
    }
}

impl FromStr for Turn {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let mut picks = value.split(" ");

        Ok(Self {
            opponent_pick: picks.next().expect("Invalid player1 pick").parse()?,
            self_pick: picks.next().expect("Invalid player2 pick").parse()?,
        })
    }
}

fn part1(score_cfg: &ScoreConfig) {
    let turns = parse_turns();
    let score = turns
        .into_iter()
        .map(|turn| turn.score(&score_cfg))
        .sum::<u32>();

    println!("{}", score);
}

struct Round {
    opponent_pick: Pick,
    turn_result: TurnResult,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Round> {
        let mut values = s.split(" ");
        Ok(Round {
            opponent_pick: values.next().expect("Invalid pick").parse()?,
            turn_result: values.next().expect("Invalid round result").parse()?,
        })
    }
}

impl Round {
    fn score(&self, score_cfg: &ScoreConfig) -> u32 {
        let self_pick = match self.turn_result {
            TurnResult::Draw => self.opponent_pick,
            TurnResult::Loser => match self.opponent_pick {
                Rock => Scissor,
                Paper => Rock,
                Scissor => Paper,
            },
            TurnResult::Winner => match self.opponent_pick {
                Rock => Paper,
                Paper => Scissor,
                Scissor => Rock,
            },
        };

        let pick_score = match self_pick {
            Rock => score_cfg.rock,
            Paper => score_cfg.paper,
            Scissor => score_cfg.scissor,
        };

        let turn_result_score = match self.turn_result {
            TurnResult::Draw => score_cfg.draw,
            TurnResult::Loser => score_cfg.loser,
            TurnResult::Winner => score_cfg.winner,
        };

        pick_score + turn_result_score
    }
}

fn part2(score_cfg: &ScoreConfig) {
    let rounds = parse_rounds();
    let score = rounds
        .into_iter()
        .map(|round| round.score(&score_cfg))
        .sum::<u32>();

    println!("{}", score);
}

fn main() {
    let score_cfg = ScoreConfig::new();
    part2(&score_cfg);
}

fn parse_rounds() -> Vec<Round> {
    std::io::stdin()
        .lines()
        .map(|try_line| try_line.expect("Invalid input"))
        .map(|line| line.parse::<Round>().expect("Failed to parse to Turn"))
        .collect()
}

fn parse_turns() -> Vec<Turn> {
    std::io::stdin()
        .lines()
        .map(|try_line| try_line.expect("Invalid input"))
        .map(|line| line.parse::<Turn>().expect("Failed to parse to Turn"))
        .collect()
}

struct ScoreConfig {
    winner: u32,
    loser: u32,
    draw: u32,
    rock: u32,
    paper: u32,
    scissor: u32,
}

impl ScoreConfig {
    fn new() -> Self {
        Self {
            winner: 6,
            loser: 0,
            draw: 3,
            rock: 1,
            paper: 2,
            scissor: 3,
        }
    }
}
