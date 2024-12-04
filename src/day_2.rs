pub fn parse_list(list: &str) -> Vec<u8> {
    list.split_whitespace()
        .map(|x| x.parse::<u8>().expect("That wasn't a number"))
        .collect()
}

pub fn is_safe(numbers: &Vec<u8>) -> bool {
    same_direction(numbers) && correct_velocity(numbers)
}

/// Returns true if all numbers are either increasing or decreasing
/// Repeated numbers are neither, return false on them
fn same_direction(list: &Vec<u8>) -> bool {
    list.is_sorted_by(|a, b| a > b) || list.is_sorted_by(|a, b| a < b)
}

/// Returns true if each adjacent number differs by at least one and at most 3
fn correct_velocity(list: &Vec<u8>) -> bool {
    list.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3)
}

pub fn dampened(numbers: &Vec<u8>) -> bool {
    // println!("Attempting to dampen: {:?}", numbers);
    numbers.iter().enumerate().any(|(i, _)| {
        let mut vec = numbers.clone();
        vec.remove(i);
        // println!("  Checking {:?}", vec);
        is_safe(&vec.to_vec())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decreasing() {
        let result = is_safe(&parse_list("7 6 4 2 1"));
        assert_eq!(true, result);
    }

    #[test]
    fn increases_too_quickly() {
        let result = is_safe(&parse_list("1 2 7 8 9"));
        assert_eq!(false, result);
    }

    #[test]
    fn decreases_too_quickly() {
        let result = is_safe(&parse_list("9 7 6 2 1"));
        assert_eq!(false, result);
    }

    #[test]
    fn direction_change() {
        let result = is_safe(&parse_list("1 3 2 4 5"));
        assert_eq!(false, result);
    }

    #[test]
    fn no_direction() {
        let result = is_safe(&parse_list("8 6 4 4 1"));
        assert_eq!(false, result);
    }

    #[test]
    fn ends_without_direction() {
        let result = is_safe(&parse_list("1 3 6 8 8"));
        assert_eq!(false, result);
    }

    #[test]
    fn increase_by_3() {
        let result = is_safe(&parse_list("1 3 6 7 9"));
        assert_eq!(true, result);
    }

    #[test]
    fn dampen_out_of_order() {
        let result = dampened(&parse_list("1 3 2 4 5"));
        assert_eq!(true, result);
    }

    #[test]
    fn cannot_dampen() {
        let result = dampened(&parse_list("9 7 6 2 1"));
        assert_eq!(false, result);
    }

    #[test]
    fn cannot_dampen_2() {
        let result = dampened(&parse_list("1 2 7 8 9"));
        assert_eq!(false, result);
    }

    #[test]
    fn dampen_repeat() {
        let result = dampened(&parse_list("8 6 4 4 1"));
        assert_eq!(true, result);
    }

    #[test]
    fn dampen_end_out_of_order() {
        let result = dampened(&parse_list("1 3 6 8 6"));
        assert_eq!(true, result);
    }
}
