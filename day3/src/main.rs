use common::parse_file;
use regex::Regex;

fn main() {
    // Parse file into vec of lines
    let text = parse_file("day3/input.txt");

    // use regex to find mul\((\d{1,3}),(\d{1,3})\)
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Should have been a valid regex");

    let start_tag = "do()";
    let stop_tag = "don't()";

    let mut sum = 0;

    let mut start = 0_usize;

    let mut stop = match text[start..].find(stop_tag).map(|i| i + start) {
        Some(stop) => stop + stop_tag.len(),
        None => text.len(),
    };

    while start < text.len() {
        let section = &text[start..stop];

        for (_, [left, right]) in re.captures_iter(section).map(|c| c.extract()) {
            sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
        }
        if stop == text.len() {
            break;
        }
        start = match text[stop..].find(start_tag).map(|i| i + stop) {
            Some(start) => start,
            None => break,
        };

        stop = match text[start..].find(stop_tag).map(|i| i + start) {
            Some(stop) => stop + stop_tag.len(),
            None => text.len(),
        };
    }

    // print the result
    println!("Sum {sum}");
}
