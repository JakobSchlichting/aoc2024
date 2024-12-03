use std::{fs::File, io::{BufRead, BufReader, Error, Lines}};

pub fn get_line_itter(path: &str) -> Result<Lines<BufReader<File>>, Error> {
    let file = std::fs::File::open(path)?;
    Ok(std::io::BufReader::new(file).lines())
}
