use day2::{is_row_safe, parse_file};

fn main() {
    let contents = parse_file("day2/input.txt");

    let mut safe_lines: i32 = 0;
    for line in contents.split("\n") {
        let numbers: Vec<i32> = line
            .split(" ")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        if is_row_safe(numbers) {
            safe_lines += 1;
        }
    }

    println!("Safe lines = {safe_lines}");
}
