
pub fn is_row_safe(row: Vec<i32>, orig_size: usize) -> bool {
    if row.is_empty() || row.len() == 1 {
        return false;
    }

    let differences = row
        .windows(2)
        .map(|values| values[0] - values[1])
        .collect::<Vec<i32>>();

    let in_range = differences
        .iter()
        .all(|&x| (1..=3).contains(&x) || (-3..=-1).contains(&x));

    let increasing = differences.iter().all(|&x| x < 0);
    let decreasing = differences.iter().all(|&x| x > 0);

    if in_range && (increasing || decreasing) {
        return true;
    }

    if orig_size == row.len() {
        // Try each other permutation
        for i in 0..orig_size {
            let mut new_guy = row.to_vec();
            new_guy.remove(i);
            if is_row_safe(new_guy, orig_size) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn known() {
        assert!(is_row_safe(vec![7, 6, 4, 2, 1], 5_usize));
        assert!(!is_row_safe(vec![1, 2, 7, 8, 9], 5_usize));
        assert!(!is_row_safe(vec![9, 7, 6, 2, 1], 5_usize));
        assert!(is_row_safe(vec![1, 3, 2, 4, 5], 5_usize));
        assert!(is_row_safe(vec![8, 6, 4, 4, 1], 5_usize));
        assert!(is_row_safe(vec![1, 3, 6, 7, 9], 5_usize));
        assert!(is_row_safe(vec![14, 11, 9, 6, 3, 2], 5_usize));
    }

    #[test]
    fn increasing() {
        assert!(is_row_safe(vec![1, 2, 3, 4], 4_usize));
        assert!(is_row_safe(vec![-1, 2, 3, 4], 4_usize));
        assert!(is_row_safe(vec![1, -2, 3, 4], 4_usize));
        assert!(is_row_safe(vec![1, 2, -3, 4], 4_usize));
        assert!(is_row_safe(vec![1, 2, 3, -4], 4_usize));
    }

    #[test]
    fn decreasing() {
        assert!(is_row_safe(vec![4, 3, 2, 1], 4_usize));
    }

    #[test]
    fn no_change() {
        assert!(!is_row_safe(vec![1, 1, 1, 1], 4_usize));
    }

    #[test]
    fn too_small() {
        assert!(!is_row_safe(vec![0], 1_usize));
    }

    #[test]
    fn increase_too_fast() {
        assert!(!is_row_safe(vec![1, 5, 10, 15], 4_usize));
    }

    #[test]
    fn decrease_too_fast() {
        assert!(!is_row_safe(vec![15, 10, 5, 1], 4_usize));
    }

    #[test]
    fn change_directions() {
        assert!(!is_row_safe(vec![15, 16, 15, 16], 4_usize));
    }
}
