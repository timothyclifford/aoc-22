use std::{
    fs::File,
    io::{self, BufRead},
};

const FILENAME: &str = "src/input.txt";

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const LOSE: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

enum TheirPlay {
    Rock,
    Paper,
    Scissors,
}
enum MyPlay {
    Lose,
    Draw,
    Win,
}

fn main() {
    let lines = read_input(FILENAME);

    let plays = lines.into_iter().filter_map(|x| parse_plays(&x));

    let scores = plays.map(parse_scores);

    let total_score: i32 = scores.sum();

    println!("Total score: {}", total_score);
}

fn parse_plays(input: &str) -> Option<(TheirPlay, MyPlay)> {
    let mut plays = input.split_whitespace();
    let their_play = parse_their_play(plays.next());
    let my_play = parse_my_play(plays.next());

    match (their_play, my_play) {
        (Some(t), Some(m)) => Some((t, m)),
        _ => None,
    }
}

fn parse_their_play(input: Option<&str>) -> Option<TheirPlay> {
    match input {
        Some(i) => match i {
            "A" => Some(TheirPlay::Rock),
            "B" => Some(TheirPlay::Paper),
            "C" => Some(TheirPlay::Scissors),
            _ => None,
        },
        _ => None,
    }
}

fn parse_my_play(input: Option<&str>) -> Option<MyPlay> {
    match input {
        Some(i) => match i {
            "X" => Some(MyPlay::Lose),
            "Y" => Some(MyPlay::Draw),
            "Z" => Some(MyPlay::Win),
            _ => None,
        },
        _ => None,
    }
}

fn parse_scores(input: (TheirPlay, MyPlay)) -> i32 {
    let base_score = match input.1 {
        MyPlay::Lose => LOSE,
        MyPlay::Draw => DRAW,
        MyPlay::Win => WIN,
    };
    let result_score = match input.0 {
        TheirPlay::Rock => match input.1 {
            MyPlay::Lose => SCISSORS,
            MyPlay::Draw => ROCK,
            MyPlay::Win => PAPER,
            _ => 0,
        },
        TheirPlay::Paper => match input.1 {
            MyPlay::Lose => ROCK,
            MyPlay::Draw => PAPER,
            MyPlay::Win => SCISSORS,
            _ => 0,
        },
        TheirPlay::Scissors => match input.1 {
            MyPlay::Lose => PAPER,
            MyPlay::Draw => SCISSORS,
            MyPlay::Win => ROCK,
            _ => 0,
        },
    };

    base_score + result_score
}

fn read_input(filename: &str) -> Vec<String> {
    let mut input_lines = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(value) = line {
                input_lines.push(value)
            }
        });
    }

    input_lines
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_their_play() {
        let input = Some("A");

        let result = parse_their_play(input);

        matches!(result.unwrap(), TheirPlay::Rock);
    }

    #[test]
    fn test_parse_my_play() {
        let input = Some("X");

        let result = parse_my_play(input);

        matches!(result.unwrap(), MyPlay::Lose);
    }

    #[test]
    fn test_parse_plays() {
        let input = "C X";

        let result = parse_plays(input);

        if let Some(r) = result {
            matches!(r.0, TheirPlay::Scissors);
            matches!(r.1, MyPlay::Lose);
        }
    }

    #[test]
    fn test_parse_scores_winning() {
        let input = (TheirPlay::Scissors, MyPlay::Win);

        let result = parse_scores(input);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_parse_scores_losing() {
        let input = (TheirPlay::Rock, MyPlay::Lose);

        let result = parse_scores(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_parse_scores_drawing() {
        let input = (TheirPlay::Paper, MyPlay::Draw);

        let result = parse_scores(input);

        assert_eq!(result, 5);
    }
}
