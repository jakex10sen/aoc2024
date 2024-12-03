use std::fs;

fn main() {
    // 1. Read from input
    let file_path = "day1/input.txt";
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // 2. Parse out pairs into lists
    let mut firsts: Vec<i32> = vec![];
    let mut seconds: Vec<i32> = vec![];
    for line in contents.split("\n") {
        // println!("{line}");
        let num_str: Vec<&str> = line.split(" ").collect();
        let first: i32 = num_str
            .first()
            .expect("Should have a first number")
            .parse()
            .expect("First wasn't a numbers");
        let second: i32 = num_str
            .last()
            .expect("Should have a last number")
            .parse()
            .expect("Last wasn't a number");
        firsts.push(first);
        seconds.push(second);
    }

    // 3. Sort each lists
    firsts.sort();
    seconds.sort();

    // 4. Calculate and sum distances
    let mut total_distance = 0;
    for i in 0..firsts.len() {
        total_distance += (firsts[i] - seconds[i]).abs();
    }
    println!("Total distance is {total_distance}");

    //5. Calculate similarities
    let mut similarity = 0;
    for first in firsts {
        let num_found = seconds.iter().filter(|&&second| first == second).count() as i32;
        similarity += first * num_found;
    }
    println!("Similarity = {similarity}")
}
