use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn extract_numbers(file_path: &str) -> Vec<i32> {
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

pub fn separate_lists(location_ids: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let list1: Vec<i32> = location_ids.iter().step_by(2).copied().collect(); 
    let list2: Vec<i32> = location_ids.iter().skip(1).step_by(2).copied().collect(); 
    (list1, list2)
}

pub fn sum_of_differences(list1: &[i32], list2: &[i32]) -> i32 {
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

pub fn find_similarity_score(list1: &[i32], list2: &[i32]) -> i32 {
    let mut list2_counts = HashMap::new();
    let mut i = 0;
    let len = list2.len();
    while i < len {
        if list2_counts.contains_key(&list2[i]) {
            let count = list2_counts.get(&list2[i]).unwrap();
            list2_counts.insert(list2[i],count + 1);
        } else {
            list2_counts.insert(list2[i],1);
        }
        i = i + 1;
    }
    let mut similarity_score = 0;
    for &ele in list1 {
        if let Some(&count) = list2_counts.get(&ele) {
            let score = &ele * &count;
            similarity_score += score;
        }
    }
    similarity_score
}
