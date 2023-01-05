use std::{
    fs::File,
    io::{BufRead, Result},
    io::{BufReader, Lines},
};

pub fn read_file_lines(filename: &str) -> Vec<String> {
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

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
