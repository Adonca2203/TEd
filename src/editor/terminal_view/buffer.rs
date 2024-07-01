use std::{
    fs::{metadata, read_to_string},
    io::{Error, ErrorKind},
};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load_file(file_name: &str) -> Result<Self, Error> {
        let md = metadata(file_name)?;
        if md.is_dir() {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!("{file_name} is a directory. Logic not yet implemented\r\n"),
            ));
        }
        let contents = read_to_string(file_name)?;

        let mut lines = Vec::new();

        for line in contents.lines() {
            lines.push(String::from(line));
        }

        Ok(Self { lines })
    }
}
