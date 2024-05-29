use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use anyhow::{Ok, Result};

pub trait LogToFile {
    fn write_to_file(&self, message: String) -> Result<()> {
        let mut file = File::create("log.txt")?;
        file.write_all(message.as_bytes())?;
        Ok(())
    }
    fn info(&self, message: String) -> Result<()> {
        self.write_to_file(message)
    }
}

pub struct Logger<'a>(&'a str);
impl<'a> Logger<'a> {
    fn write_to_file(&self, message: String) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.0)
            .unwrap();
        writeln!(file, "{}", message)?;
        Ok(())
    }
    pub fn info(&self, message: String) -> Result<()> {
        self.write_to_file(message)
    }
    pub fn change_file(&mut self, log_file: &'a str) {
        self.0 = log_file;
    }
}

pub static LOGGER: Logger = Logger("log.txt");
