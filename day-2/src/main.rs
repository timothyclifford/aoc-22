use std::{
    fs::File,
    io::{self, BufRead},
};

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

enum Play {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    const FILENAME: &str = "src/input.txt";

    let lines = read_input(FILENAME);

    let plays = lines.into_iter().filter_map(|x| parse_plays(&x));

    let scores = plays.map(parse_scores);

    let total_score: i32 = scores.sum();

    println!("Total score: {}", total_score);
}

fn parse_plays(input: &str) -> Option<(Play, Play)> {
    let mut plays = input.split_whitespace();
    let their_play = parse_play(plays.next());
    let my_play = parse_play(plays.next());

    match (their_play, my_play) {
        (Some(t), Some(m)) => Some((t, m)),
        _ => None,
    }
}

fn parse_play(input: Option<&str>) -> Option<Play> {
    match input {
        Some(i) => match i {
            "A" | "X" => Some(Play::Rock),
            "B" | "Y" => Some(Play::Paper),
            "C" | "Z" => Some(Play::Scissors),
            _ => None,
        },
        _ => None,
    }
}

fn parse_scores(input: (Play, Play)) -> i32 {
    let base_score = match input.1 {
        Play::Rock => ROCK,
        Play::Paper => PAPER,
        Play::Scissors => SCISSORS,
    };
    let result_score = match input.0 {
        Play::Rock => match input.1 {
            Play::Rock => 3,
            Play::Paper => 6,
            Play::Scissors => 0,
            _ => 0,
        },
        Play::Paper => match input.1 {
            Play::Rock => 0,
            Play::Paper => 3,
            Play::Scissors => 6,
            _ => 0,
        },
        Play::Scissors => match input.1 {
            Play::Rock => 6,
            Play::Paper => 0,
            Play::Scissors => 3,
            _ => 0,
        },
    };

    base_score + result_score
}

fn read_input(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();

    if let Ok(linez) = read_lines(filename) {
        linez.for_each(|line| {
            if let Ok(value) = line {
                lines.push(value)
            }
        });
    }

    lines
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_play() {
        let input = Some("A");

        let result = parse_play(input);

        matches!(result.unwrap(), Play::Rock);
    }

    #[test]
    fn test_parse_plays() {
        let input = "C X";

        let result = parse_plays(input);

        if let Some(r) = result {
            matches!(r.0, Play::Scissors);
            matches!(r.1, Play::Rock);
        }
    }

    #[test]
    fn test_parse_scores_winning() {
        let input = (Play::Scissors, Play::Rock);

        let result = parse_scores(input);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_parse_scores_losing() {
        let input = (Play::Rock, Play::Scissors);

        let result = parse_scores(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_parse_scores_drawing() {
        let input = (Play::Paper, Play::Paper);

        let result = parse_scores(input);

        assert_eq!(result, 5);
    }
}
