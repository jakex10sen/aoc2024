use std::fs;

pub fn parse_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file {file_path}")
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
// }
