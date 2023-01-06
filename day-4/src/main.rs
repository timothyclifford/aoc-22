mod file_reader;

const FILENAME: &str = "src/input";

struct Assignments {
    pairs: Vec<Pair>,
}

impl Assignments {
    fn new(pairs: Vec<Pair>) -> Self {
        Self { pairs }
    }
}

struct Pair {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Pair {
    fn new(left: Vec<i32>, right: Vec<i32>) -> Self {
        Self { left, right }
    }

    fn any_match(&self) -> bool {
        self.left.iter().any(|xx| self.right.contains(xx))
            || self.right.iter().any(|xx| self.left.contains(xx))
    }

    fn is_covered(&self) -> bool {
        self.left.iter().all(|xx| self.right.contains(xx))
            || self.right.iter().all(|xx| self.left.contains(xx))
    }
}

fn main() {
    let lines = file_reader::read_file_lines(FILENAME);

    let pairs: Vec<(String, String)> = lines
        .into_iter()
        .map(|line| {
            let ranges = line.split(',').collect::<Vec<_>>();
            (
                ranges.first().unwrap().to_string(),
                ranges.last().unwrap().to_string(),
            )
        })
        .collect();

    let pairs_with_ranges: Vec<(Vec<i32>, Vec<i32>)> = pairs
        .into_iter()
        .map(|pair| (expand_ranges(pair.0), expand_ranges(pair.1)))
        .collect();

    let assignments = Assignments::new(
        pairs_with_ranges
            .into_iter()
            .map(|pair| Pair::new(pair.0, pair.1))
            .collect(),
    );

    let number_of_pairs_contained: i32 = assignments
        .pairs
        .iter()
        .map(|pair| {
            return if pair.any_match() { 1 } else { 0 };
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!(
        "{} pairs are completely contained",
        number_of_pairs_contained
    );
}

fn expand_ranges(raw: String) -> Vec<i32> {
    let mut ranges = Vec::new();
    let ranges_start_end: Vec<i32> = raw.split('-').flat_map(|x| x.parse::<i32>()).collect();
    let start = ranges_start_end.first().unwrap().to_owned();
    let end = ranges_start_end.last().unwrap().to_owned();

    for i in start..=end {
        ranges.push(i)
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_ranges() {
        let result = expand_ranges(String::from("1-23"));

        assert_eq!(result.len(), 23);
        assert_eq!(result.contains(&1), true);
        assert_eq!(result.contains(&23), true);
    }

    #[test]
    fn test_is_covered_true() {
        let pair = Pair::new(vec![1, 2, 3, 4], vec![2, 3, 4]);

        let result = pair.is_covered();

        assert_eq!(result, true);
    }

    #[test]
    fn test_is_covered_false() {
        let pair = Pair::new(vec![1, 2, 3, 4, 42], vec![2, 3, 4, 21]);

        let result = pair.is_covered();

        assert_eq!(result, false);
    }

    #[test]
    fn test_any_match_true() {
        let pair = Pair::new(vec![1, 2, 3, 4], vec![2, 3, 4, 5]);

        let result = pair.any_match();

        assert_eq!(result, true);
    }

    #[test]
    fn test_any_match_false() {
        let pair = Pair::new(vec![1, 2, 3, 4], vec![5, 6, 7]);

        let result = pair.any_match();

        assert_eq!(result, false);
    }
}
