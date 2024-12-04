use std::fs;

pub fn parse_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file {file_path}")
}

pub fn is_row_safe(row: Vec<i32>) -> bool {
    if row.is_empty() {
        return false;
    } else if row.len() == 1 {
        return false;
    }

    let mut increasing: bool = false;
    for i in 1..row.len() {
        let first: i32 = row[i - 1];
        let second: i32 = row[i];
        let diff: i32 = first - second;

        if diff == 0 {
            return false;
        }

        if i == 1 {
            if diff > 0 {
                increasing = true;
            } else {
                increasing = false;
            }
        }

        if increasing {
            if diff > 3 || diff < 1 {
                return false;
            }
        } else {
            if diff < -3 || diff > -1 {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn increasing() {
        assert!(is_row_safe(vec![1, 2, 3, 4]));
    }

    #[test]
    fn decreasing() {
        assert!(is_row_safe(vec![4, 3, 2, 1]));
    }

    #[test]
    fn no_change() {
        assert_eq!(is_row_safe(vec![1, 1, 1, 1]), false);
    }

    #[test]
    fn too_small() {
        assert_eq!(is_row_safe(vec![0]), false);
    }

    #[test]
    fn increase_too_fast() {
        assert_eq!(is_row_safe(vec![1, 5, 10, 15]), false);
    }

    #[test]
    fn decrease_too_fast() {
        assert_eq!(is_row_safe(vec![15, 10, 5, 1]), false);
    }

    #[test]
    fn change_directions() {
        assert_eq!(is_row_safe(vec![15, 16, 15, 16]), false);
    }


}
