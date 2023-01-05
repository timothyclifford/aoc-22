use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
};

const FILENAME: &str = "src/input.txt";

fn main() {
    let priorities = init_priorities();
    let lines = read_input(FILENAME);

    do_activity_one(&priorities, &lines);
    do_activity_two(&priorities, &lines);
}

fn do_activity_one(priorities: &HashMap<char, i32>, lines: &Vec<String>) {
    let compartments = lines.iter().map(|l| get_compartments(l));

    let compartment_shared_items = compartments
        .flat_map(|(c1, c2)| find_compartment_shared_items(c1, c2))
        .collect();

    let answer = sum_item_priorities(priorities, compartment_shared_items);

    println!("ACTIVITY ONE: {}", answer);
}

fn do_activity_two(priorities: &HashMap<char, i32>, lines: &Vec<String>) {
    let groups = get_groups(lines);

    let group_shared_items = groups
        .iter()
        .flat_map(|x| {
            find_group_shared_items(x.get(0).unwrap(), x.get(1).unwrap(), x.get(2).unwrap())
        })
        .collect();

    let answer = sum_item_priorities(priorities, group_shared_items);

    println!("ACTIVITY TWO: {}", answer);
}

fn get_groups(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();

    for line in lines.iter() {
        group.push(line.to_string());

        if group.len() == 3 {
            groups.push(group);
            group = Vec::new();
        }
    }

    if group.len() > 0 && group.len() < 3 {
        groups.push(group);
    }

    groups
}

fn get_compartments(contents: &str) -> (Vec<char>, Vec<char>) {
    let contents_size = contents.len();
    let compartments = contents.split_at(contents_size / 2);

    (
        compartments.0.chars().collect(),
        compartments.1.chars().collect(),
    )
}

fn find_compartment_shared_items(
    compartment_one: Vec<char>,
    compartment_two: Vec<char>,
) -> Vec<char> {
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

fn find_group_shared_items(elf1: &str, elf2: &str, elf3: &str) -> Option<char> {
    let mut shared_items: HashMap<char, char> = HashMap::new();

    for item in elf2.chars().into_iter() {
        if let Some(_) = elf1.chars().into_iter().find(|&x| x == item) {
            shared_items.insert(item, item);
        }
    }

    for item in elf3.chars().into_iter() {
        if let Some(_) = shared_items.iter().find(|(&k, v)| k == item) {
            return Some(item);
        }
    }

    return None;
}

fn sum_item_priorities(priorities: &HashMap<char, i32>, items: Vec<char>) -> i32 {
    items.iter().flat_map(|item| priorities.get(item)).sum()
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
    fn test_get_groups() {
        let lines = Vec::from([
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
            String::from("jkl"),
            String::from("mno"),
            String::from("pqr"),
        ]);
        let result = get_groups(&lines);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_get_compartments() {
        let result = get_compartments("vJrwpWtwJgWrhcsFMMfFFhFp");

        assert_eq!(result.0, "vJrwpWtwJgWr".chars().collect::<Vec<_>>());
        assert_eq!(result.1, "hcsFMMfFFhFp".chars().collect::<Vec<_>>());
    }

    #[test]
    fn test_find_compartment_shared_items() {
        let result = find_compartment_shared_items(
            "vJrwpWtwJgWr".chars().collect(),
            "hcsFMMfFFhFp".chars().collect(),
        );

        assert_eq!(result, vec!['p']);
    }

    #[test]
    fn test_find_group_shared_items() {
        let result = find_group_shared_items(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        );

        assert_eq!(result.unwrap(), 'r');
    }

    #[test]
    fn test_sum_item_priorities() {
        let priorities = init_priorities();
        let shared_items = vec!['a', 'A', 'z', 'Z'];

        let result = sum_item_priorities(&priorities, shared_items);

        assert_eq!(result, 1 + 26 + 27 + 52);
    }
}
