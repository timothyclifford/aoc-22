use std::{
    fs::File,
    io::{BufRead, Result},
    io::{BufReader, Lines},
};

pub struct FileReader {
    filename: String,
}

impl FileReader {
    pub fn new() -> FileReader {
        FileReader {
            filename: String::from("src/input"),
        }
    }

    pub fn set_filename<'a>(&'a mut self, filename: String) -> &'a mut FileReader {
        self.filename = filename;
        self
    }

    pub fn read_file_lines(&self) -> Vec<String> {
        let mut input_lines = Vec::new();

        if let Ok(lines) = self.read_lines() {
            lines.for_each(|line| {
                if let Ok(value) = line {
                    input_lines.push(value)
                }
            });
        }

        input_lines
    }

    fn read_lines(&self) -> Result<Lines<BufReader<File>>> {
        let file = File::open(self.filename.as_str())?;
        Ok(BufReader::new(file).lines())
    }
}
