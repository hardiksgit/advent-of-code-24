use std::fs::File;
use std::io::{self, BufRead};

pub fn extract_reports(file_path: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    if let Ok(file) = File::open(file_path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(content) = line {
                let numbers: Vec<i32> = content
                    .split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect();
                reports.push(numbers);
            }
        }
    } else {
        eprintln!("Could not open the file at path: {}", file_path); 
    }
    reports
}

pub fn find_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        if is_report_safe(report) {
            safe_reports += 1;
        } else {
            if apply_problem_dampener(report) {
                safe_reports += 1;
            }
        }
    }
    safe_reports
}

fn apply_problem_dampener(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut cloned_report = report.clone();
        cloned_report.remove(i);

        if is_report_safe(&cloned_report) {
            return true;
        }
    }
    false
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return  false;
    }
    let is_increasing = report[1] > report[0];
    for i in 1..report.len() {
        let diff = report[i] - report[i-1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if is_increasing && diff <= 0 || !is_increasing && diff >= 0 {
            return false;
        }
    }
    true
}