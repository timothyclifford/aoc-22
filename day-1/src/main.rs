use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    const FILENAME: &str = "src/input.txt";

    let lines = read_input(FILENAME);
    let elf_calories = get_elf_calories(lines);

    // let elf_with_most_calories = find_elf_with_most_calories(elf_calories);

    let elves_most_calories = find_3_elves_with_most_calories(elf_calories);

    println!("Calories: {}", elves_most_calories.iter().sum::<i32>());
}

fn read_input(filename: &str) -> Vec<String> {
    let mut elves = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(value) = line {
                elves.push(value)
            }
        });
    }

    elves
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_elf_calories(elves: Vec<String>) -> Vec<Vec<i32>> {
    let mut current_elf = Vec::new();
    let mut all_elves = Vec::new();

    for elf in elves.iter() {
        if elf.trim().is_empty() {
            all_elves.push(current_elf);
            current_elf = Vec::new();
        } else if let Ok(calories) = elf.parse::<i32>() {
            current_elf.push(calories);
        } else {
            println!("Failed to parse calories value {}", elf)
        }
    }

    all_elves.push(current_elf);

    all_elves
}

fn find_elf_with_most_calories(elves: Vec<Vec<i32>>) -> i32 {
    let mut most = 0;

    for elf in elves.iter() {
        let calories = elf.iter().sum();
        if calories > most {
            most = calories;
        }
    }

    most
}

fn find_3_elves_with_most_calories(elves: Vec<Vec<i32>>) -> Vec<i32> {
    let mut calories = Vec::<i32>::new();

    for elf in elves.iter() {
        calories.push(elf.iter().sum());
    }

    calories.sort();

    return calories.iter().rev().take(3).cloned().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILENAME_TEST: &str = "/home/timothyclifford/work/aoc-22/day-1/src/input-test.txt";

    #[test]
    fn test_read_input() {
        let input = FILENAME_TEST;
        let expected = vec![
            String::from("123"),
            String::from(""),
            String::from("456"),
            String::from("321"),
            String::from(""),
            String::from("228"),
        ];

        let result = read_input(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_elf_calories() {
        let elves = vec![
            String::from("123"),
            String::from(""),
            String::from("456"),
            String::from("321"),
            String::from(""),
            String::from("228"),
        ];
        let expected = vec![vec![123], vec![456, 321], vec![228]];

        let result = get_elf_calories(elves);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_elf_with_most_calories() {
        let elves = vec![vec![123], vec![456, 321], vec![228]];
        let expected = 777;

        let result = find_elf_with_most_calories(elves);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_3_elves_with_most_calories() {
        let elves = vec![vec![123], vec![456, 321], vec![228]];
        let expected = vec![777, 228, 123];

        let result = find_3_elves_with_most_calories(elves);

        assert_eq!(result, expected);
    }
}
