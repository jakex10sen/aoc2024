use std::fs;

use regex::Regex;

pub fn parse_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file {file_path}")
}
fn main() {
    // Parse file into vec of lines
    let text = parse_file("day3/input.txt");

    // use regex to find mul\((\d{1,3}),(\d{1,3})\)
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Should have been a valid regex");

    let mut sum = 0;
    for (_, [left, right]) in re.captures_iter(&text).map(|c| c.extract()) {
        // Multiply capture group 1 by capture group 2 and sum the results
        sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
    }
    // print the result
    println!("Sum {sum}");
}
