mod day1;
mod day2;
mod day3;
use day1::{extract_locations, find_similarity_score, separate_lists, sum_of_differences};
use day2::{find_safe_reports, extract_reports};
use day3::mull_it_over;

fn main() {
    let location_file_path = "/Users/hardik/Desktop/advent-of-code-24/src/inputs/location_list.txt"; 
    let location_ids = extract_locations(&location_file_path);
    let (mut list1, mut list2) = separate_lists(&location_ids);
    list1.sort();
    list2.sort();
    sum_of_differences(&list1, &list2);
    find_similarity_score(&list1, &list2);
    let reports_file_path = "/Users/hardik/Desktop/advent-of-code-24/src/inputs/reports.txt";
    let reports = extract_reports(&reports_file_path);
    find_safe_reports(&reports);
    let corrupted_input_file_path = "/Users/hardik/Desktop/advent-of-code-24/src/inputs/corrupted_input.txt";
    mull_it_over(&corrupted_input_file_path);
}