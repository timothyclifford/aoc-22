use std::{
    fs::File,
    io::{self, BufRead},
};

type Point = (i32, char);

const FILENAME: &str = "src/input.txt";

fn main() {
    let priorities = init_priorities();

    let lines = read_input(FILENAME);

    let compartments = lines.iter().map(|l| get_compartments(l));

    let shared_items = compartments.map(|(c1, c2)| todo!());
    // read lines
    // iterate through
    // get compartment contents
    // find shared items
    // sum shared item priorities
    // println!("{}", alphabet);
}

fn init_priorities() -> std::vec::Vec<Point> {
    let alphabet = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic());

    alphabet
        .enumerate()
        .map(|(i, x)| -> Point { (i as i32 + 1, x) })
        .collect()
}

fn get_compartments(contents: &str) -> (Vec<char>, Vec<char>) {
    let contents_size = contents.len();
    let compartments = contents.split_at(contents_size / 2);

    (
        compartments.0.chars().collect::<Vec<_>>(),
        compartments.1.chars().collect::<Vec<_>>(),
    )
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

        assert_eq!(result.first().unwrap().0, 1);
        assert_eq!(result.first().unwrap().1, 'a');
        assert_eq!(result.get(25).unwrap().0, 26);
        assert_eq!(result.get(25).unwrap().1, 'z');
        assert_eq!(result.get(26).unwrap().0, 27);
        assert_eq!(result.get(26).unwrap().1, 'A');
        assert_eq!(result.last().unwrap().0, 52);
        assert_eq!(result.last().unwrap().1, 'Z');
    }

    #[test]
    fn test_get_compartments() {
        let result = get_compartments("vJrwpWtwJgWrhcsFMMfFFhFp");

        assert_eq!(result.0, "vJrwpWtwJgWr".chars().collect::<Vec<_>>());
        assert_eq!(result.1, "hcsFMMfFFhFp".chars().collect::<Vec<_>>());
    }
}
