fn main() {
    let input = include_str!("../../inputs/day02.txt");

    let total_score = input.lines().fold(0, |total_score, line| {
        let mut score = 0;
        let actions = line.split_whitespace().map(get_action_for_str).collect::<Vec<Action>>();
        score += get_points_for_action(&actions[1]);
        score += get_points_for_outcome(&actions[0], &actions[1]);
        total_score + score
    });
    println!("Total score: {}", total_score);
}

fn get_action_for_str(c: &str) -> Action {
    match c {
        "A" | "X" => Action::Rock,
        "B" | "Y" => Action::Paper,
        "C" | "Z" => Action::Scissors,
        _ => panic!("Unknown action '{}'", c),
    }
}

fn get_points_for_action(action: &Action) -> u32 {
    match action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    }
}

fn get_points_for_outcome(other_action: &Action, own_action: &Action) -> u32 {
    if other_action == own_action {
        // Draw
        return 3;
    }
    match (other_action, own_action) {
        // Win
        (Action::Rock, Action::Paper) => 6,
        (Action::Paper, Action::Scissors) => 6,
        (Action::Scissors, Action::Rock) => 6,
        // Lose
        (Action::Rock, Action::Scissors) => 0,
        (Action::Paper, Action::Rock) => 0,
        (Action::Scissors, Action::Paper) => 0,
        _ => panic!("Unknown outcome"),
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}