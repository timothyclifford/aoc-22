use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
};

const FILENAME: &str = "src/input.txt";

fn main() {
    let priorities = init_priorities();

    let lines = read_input(FILENAME);

    let compartments = lines.iter().map(|l| get_compartments(l));

    let shared_items = compartments
        .flat_map(|(c1, c2)| find_shared_items(c1, c2))
        .collect();

    let answer = sum_shared_item_priorities(priorities, shared_items);

    println!("ANSWER: {}", answer);
}

fn init_priorities() -> HashMap<char, i32> {
    let alphabet = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic());

    alphabet
        .enumerate()
        .map(|(i, x)| (x, i as i32 + 1))
        .collect()
}

fn get_compartments(contents: &str) -> (Vec<char>, Vec<char>) {
    let contents_size = contents.len();
    let compartments = contents.split_at(contents_size / 2);

    (
        compartments.0.chars().collect(),
        compartments.1.chars().collect(),
    )
}

fn find_shared_items(compartment_one: Vec<char>, compartment_two: Vec<char>) -> Vec<char> {
    let mut shared_items = Vec::new();

    for compartment in compartment_one {
        if let Some(found) = compartment_two.iter().find(|&&x| x == compartment) {
            if !shared_items.contains(found) {
                shared_items.push(*found);
            }
        }
    }

    shared_items
}

fn sum_shared_item_priorities(priorities: HashMap<char, i32>, shared_items: Vec<char>) -> i32 {
    shared_items
        .iter()
        .flat_map(|item| priorities.get(item))
        .sum()
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
    fn test_init_priorities() {
        let result = init_priorities();

        assert_eq!(result.get(&'a').unwrap().to_owned(), 1_i32);
    }

    #[test]
    fn test_get_compartments() {
        let result = get_compartments("vJrwpWtwJgWrhcsFMMfFFhFp");

        assert_eq!(result.0, "vJrwpWtwJgWr".chars().collect::<Vec<_>>());
        assert_eq!(result.1, "hcsFMMfFFhFp".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_find_shared_items() {
        let result = find_shared_items(
            "vJrwpWtwJgWr".chars().collect(),
            "hcsFMMfFFhFp".chars().collect(),
        );

        assert_eq!(result, vec!['p']);
    }

    #[test]
    fn test_sum_shared_item_priorities() {
        let priorities = init_priorities();
        let shared_items = vec!['a', 'A', 'z', 'Z'];

        let result = sum_shared_item_priorities(priorities, shared_items);

        assert_eq!(result, 1 + 26 + 27 + 52);
    }
}
