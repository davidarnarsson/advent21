use std::fs;
use std::io::Result;



pub fn read_file(path: &str) -> Result<Vec<String>> {
    fs::read_to_string(path).and_then(|s| Ok(s.split("\n").map(|f| f.to_owned()).collect()))
}
