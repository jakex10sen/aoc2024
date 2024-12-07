use day2::{is_row_safe, parse_file};

fn main() {
    let contents = parse_file("day2/input.txt");

    let mut safe_lines: i32 = 0;
    for line in contents.split("\n") {
        let numbers: Vec<i32> = line
            .split(" ")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let len = numbers.len();
        let num_str = format!("{:?}", numbers);
        if is_row_safe(numbers, len) {
            safe_lines += 1;
        } else {
            println!("Not {:?}", num_str);
        }
    }

    println!("Safe lines = {safe_lines}");
}
