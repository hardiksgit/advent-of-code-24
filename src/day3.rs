use std::fs::File;
use std::io::{BufRead, self};
use regex::Regex;

pub fn mull_it_over(file_path: &str) -> i32 {
    let mut sum = 0;
    if let Ok(file) = File::open(file_path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(content) = line {
                let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                for captured in re.captures_iter(&content) {
                    let num1: i32 = captured[1].parse().unwrap();
                    let num2: i32 = captured[2].parse().unwrap();
                    sum += num1 * num2;
                }
            }
        }
    } else {
        eprint!("could not open the file at path {} ", file_path);
    }
    sum
}