use std::fs::File;
use std::io::{self, BufRead};

fn extract_numbers(file_path: &str) -> Vec<i32> {
    let mut location_ids = Vec::new();
    if let Ok(file) = File::open(file_path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(content) = line {
                for num_str in content.split_whitespace() {
                    if let Ok(num) = num_str.parse::<i32>() {
                        location_ids.push(num);
                    }
                }
            }
        }
    } else {
        eprintln!("Could not open the file at path: {}", file_path); 
    }
    location_ids
}

fn separate_lists(location_ids: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let list1: Vec<i32> = location_ids.iter().step_by(2).copied().collect(); 
    let list2: Vec<i32> = location_ids.iter().skip(1).step_by(2).copied().collect(); 
    (list1, list2)
}

fn sum_of_differences(list1: &[i32], list2: &[i32]) -> i32 {
    let len = list1.len().min(list2.len());
    let mut sum = 0;
    let mut i = 0;
    while i < len {
        let diff = (list1[i] - list2[i]).abs();
        sum = sum + diff;
        i = i + 1;
    }
    sum
}

fn main() {
    let file_path = "/Users/hardik/Desktop/advent-of-code-24/src/location_list.txt"; 
    let location_ids = extract_numbers(file_path);
    let (mut list1, mut list2) = separate_lists(&location_ids);
    list1.sort();
    list2.sort();
    let total_difference = sum_of_differences(&list1, &list2);
}