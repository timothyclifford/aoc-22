mod file_reader;

struct Supplies {
    stacks: Vec<Stack>,
}
struct Stack {
    name: String,
    crates: Vec<Crate>,
}
struct Crate {
    name: String,
}

fn main() {
    let lines = file_reader::FileReader::new().read_file_lines();

    // get stacks by reading lines until blank found
    // parse stacks
    // read remaining lines to get moves
    // play moves
    // join top crate in each stack
}

fn get_raw_stacks(lines: Vec<String>) -> Vec<String> {
    let mut stacks = Vec::new();
    let mut iterator = lines.iter();

    let mut optional = iterator.next();

    while let Some(line) = optional {
        if line.is_empty() {
            optional = None;
        } else {
            stacks.push(line.to_string());
            optional = iterator.next();
        }
    }

    stacks.reverse();

    stacks
}

fn parse_raw_stacks(raw: Vec<String>) -> Supplies {
    // let max_length = raw
    //     .iter()
    //     .max_by(|x, y| x.len().cmp(&y.len()))
    //     .unwrap()
    //     .len()
    //     + 1;

    // let number_of_stacks = max_length / 4;

    let number_of_stacks = get_number_of_stacks(raw.first().unwrap());

    let mut stacks: Vec<Stack> = Vec::with_capacity(number_of_stacks);

    for line in raw.iter().skip(1) {
        let chars: Vec<char> = line.chars().collect();
    }
    // get number of stacks and create
    // iterate through remaining lines
    // parse crates, put in stacks

    Supplies { stacks }
}

fn get_number_of_stacks(raw: &str) -> usize {
    let number = raw
        .chars()
        .reduce(|acc: char, x: char| {
            return if x.is_numeric() && (x as i32) > (acc as i32) {
                x
            } else {
                acc
            };
        })
        .unwrap()
        .to_digit(10)
        .unwrap();

    usize::try_from(number).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_raw_stacks() {
        let input = Vec::from([
            String::from("[D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
            String::from(""),
            String::from("move 1 from 2 to 1"),
            String::from("move 3 from 1 to 3"),
            String::from("move 2 from 2 to 1"),
            String::from("move 1 from 1 to 2"),
        ]);
        let expected = Vec::from([
            String::from("[D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
        ]);

        let result = get_raw_stacks(input);

        assert_eq!(result.iter().len(), expected.iter().len());
    }

    #[test]
    fn test_parse_raw_stacks() {
        let input = Vec::from([
            String::from("[D]    "),
            String::from("[N] [C]    "),
            String::from("[Z] [M] [P]"),
            String::from(" 1   2   3 "),
        ]);

        let result = parse_raw_stacks(input);

        assert_eq!(result.stacks.len(), 3);
        assert_eq!(result.stacks.get(0).unwrap().crates.len(), 3);
        assert_eq!(result.stacks.get(1).unwrap().crates.len(), 2);
        assert_eq!(result.stacks.get(2).unwrap().crates.len(), 1);
    }

    #[test]
    fn test_get_number_of_stacks() {
        let input = " 1   2   3 ";

        let result = get_number_of_stacks(input);

        assert_eq!(result, 3);
    }
}
