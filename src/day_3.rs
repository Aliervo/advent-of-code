pub fn sum_of_multiplied(vec: Vec<(u32, u32)>) -> u32 {
    // println!("Multiplying and adding: {:?}", vec);
    vec.iter().fold(0, |acc, (x, y)| acc + x * y)
}

pub fn find_tuples(string: &str) -> Vec<(u32, u32)> {
    string.split("mul(").fold(Vec::new(), |mut acc, seq| {
        let end_of_seq = seq.find(|c: char| !c.is_ascii_digit() && c != ',').unwrap();
        if seq.starts_with(|c: char| c.is_ascii_digit())
            && seq.chars().nth(end_of_seq).unwrap() == ')'
        {
            let (first, _) = seq.split_at(end_of_seq);
            let nums: Vec<u32> = first
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let tup = (nums[0], nums[1]);
            acc.push(tup);
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_a_tuple() {
        let result = find_tuples("xmul(2,4)");
        assert_eq!(result, vec![(2, 4)]);
    }

    #[test]
    fn it_knows_brackets() {
        let result = find_tuples("do_not_mul(5,5)+mul(32,64]");
        assert_eq!(result, vec![(5, 5)]);
    }

    #[test]
    fn it_can_math() {
        let result = sum_of_multiplied(vec![(1, 4), (2, 5), (3, 3)]);
        assert_eq!(result, 23);
    }

    #[test]
    fn it_works_with_gt_9() {
        let result = find_tuples("then(mul(11,8)mul(8,5))");
        assert_eq!(result, vec![(11, 8), (8, 5)]);
    }
}
