use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/day02.txt");

    part_1(input);
    part_2(input);
}

fn part_2(input: &str) {
    let total_score = input.lines().fold(0, |total_score, line| {
        let mut score = 0;
        let actions = line.split_whitespace().collect::<Vec<&str>>();
        let action = str::parse(&actions[0]).unwrap();
        let outcome = str::parse(&actions[1]).unwrap();
        score += get_points_for_set_outcome(&action, &outcome);
        total_score + score
    });
    println!("[PART2] Total score: {}", total_score)
}

fn part_1(input: &str) {
    let total_score = input.lines().fold(0, |total_score, line| {
        let mut score = 0;
        let actions = line
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<Action>>();
        score += actions[1].get_value();
        score += get_outcome_for_actions(&actions[0], &actions[1]).get_value();
        total_score + score
    });
    println!("[PART1] Total score: {}", total_score);
}

fn get_points_for_set_outcome(other_action: &Action, outcome: &Outcome) -> u32 {
    outcome.get_value()
        + match outcome {
            Outcome::Win => match other_action {
                Action::Rock => Action::Paper.get_value(),
                Action::Paper => Action::Scissors.get_value(),
                Action::Scissors => Action::Rock.get_value(),
            },
            Outcome::Lose => match other_action {
                Action::Rock => Action::Scissors.get_value(),
                Action::Paper => Action::Rock.get_value(),
                Action::Scissors => Action::Paper.get_value(),
            },
            Outcome::Draw => match other_action {
                Action::Rock => Action::Rock.get_value(),
                Action::Paper => Action::Paper.get_value(),
                Action::Scissors => Action::Scissors.get_value(),
            },
        }
}

fn get_outcome_for_actions(other_action: &Action, own_action: &Action) -> Outcome {
    if other_action == own_action {
        // Draw
        return Outcome::Draw;
    }
    match (other_action, own_action) {
        // Win
        (Action::Rock, Action::Paper)
        | (Action::Paper, Action::Scissors)
        | (Action::Scissors, Action::Rock) => Outcome::Win,
        // Lose
        (Action::Rock, Action::Scissors)
        | (Action::Paper, Action::Rock)
        | (Action::Scissors, Action::Paper) => Outcome::Lose,
        _ => panic!("Unknown outcome"),
    }
}

trait GetValue {
    fn get_value(&self) -> u32;
}

#[derive(Debug, PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Action::Rock),
            "B" | "Y" => Ok(Action::Paper),
            "C" | "Z" => Ok(Action::Scissors),
            _ => Err(()),
        }
    }
}

impl GetValue for Action {
    fn get_value(&self) -> u32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl GetValue for Outcome {
    fn get_value(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}
