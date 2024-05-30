use std::{fs::OpenOptions, io::Write};

use anyhow::{Ok, Result};

pub trait LogToFile {
    fn write_to_file(&self, message: &str) -> Result<()> {
        LOGGER.write_to_file(message)
    }
    fn info(&self, message: &str) -> Result<()> {
        self.write_to_file(message)
    }
}

pub struct Logger<'a>(&'a str);
impl<'a> Logger<'a> {
    pub fn write_to_file(&self, message: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.0)
            .unwrap();
        writeln!(file, "{}", message)?;
        Ok(())
    }
    pub fn info(&self, message: &str) -> Result<()> {
        self.write_to_file(message)
    }
    pub fn change_file(&mut self, log_file: &'a str) {
        self.0 = log_file;
    }
}

pub static LOGGER: Logger = Logger("log.txt");
