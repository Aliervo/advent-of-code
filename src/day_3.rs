// use std::iter;

pub fn sum_of_multiplied(vec: Vec<(u32, u32)>) -> u32 {
    // println!("Multiplying and adding: {:?}", vec);
    vec.iter().fold(0, |acc, (x, y)| acc + x * y)
}

pub fn find_tuples(string: &str) -> Vec<(u32, u32)> {
    // Remove disabled parts of the string
    // let mut pruned = String::from(string);
    // Check if anything needs removing
    // if let Some(first_dont) = string.find("don't()") {
    let mut donts: Vec<_> = string
        .match_indices("don't()")
        .chain(string.match_indices("do()"))
        .collect();
    // println!("{:?}", donts.clone().collect::<Vec<_>>());
    // let dos_after_donts = format!(
    //     "{}{}",
    //     vec!["x"; first_dont].join(""),
    //     string.get(first_dont..).unwrap()
    // );
    // let dos: Vec<_> = string.match_indices("do()").collect();
    // println!("{:?}", dos.clone().collect::<Vec<_>>());

    // donts.extend(dos);

    donts.sort_by(|(a, _), (b, _)| a.cmp(b));
    let deduped = donts.iter().fold(Vec::new(), |mut acc, (pos, name)| {
        match acc.pop() {
            None => {
                // Push the first don't
                if *name == "don't()" {
                    acc.push((pos, name));
                }
            }
            Some(last) => {
                let (_, prev) = last;
                if name != prev {
                    acc.push(last);
                    acc.push((pos, name));
                } else {
                    acc.push(last);
                }
            }
        }
        acc
    });

    let active = deduped.iter().fold(Vec::new(), |mut acc, (&pos, _)| {
        match acc.pop() {
            None => {
                acc.push((0, pos));
            }
            Some(last) => {
                let (start, end) = last;
                match end {
                    0 => {
                        acc.push((start, pos));
                    }
                    _ => {
                        acc.push(last);
                        acc.push((pos, 0));
                    }
                }
            }
        }
        acc
    });

    let pruned = active
        .iter()
        .fold(String::new(), |mut acc: String, (start, end)| {
            if end != &0 {
                let chunk = string.get(*start..*end).expect("You let it end with 0");
                acc.push_str(chunk);
            } else {
                let tail = string.get(*start..).unwrap();
                acc.push_str(tail);
            }
            acc
        });
    // println!("{:?}", pruned);

    // let pruned = donts.fold(String::from(string), |mut acc: String, (start, _)| {
    //     // println!("Slicing from:\n {string}");
    //     match dos.find(|(x, _)| x > &start) {
    //         Some(tuple) => {
    //             let (stop, _) = tuple;
    //             let to_remove = string
    //                 .get(start..stop + 4)
    //                 .expect("Do list was longer than don't");
    //             // println!("{to_remove}");
    //             acc = acc.replace(to_remove, "");
    //         }
    //         None => {
    //             let to_remove = string.get(start..).unwrap();
    //             // println!("Last bit to prune: {to_remove}");
    //             acc = acc.replace(to_remove, "");
    //         }
    //     }
    //     // println!("{acc}");
    //     acc
    // });
    //
    // Pad the do iter, as it should always be shorter
    // let zipper = donts.zip(dos.chain(iter::repeat((0, "Nothing"))));

    // This strategy is currently borked.
    // When I mutate the acc and try to use it as the base string to slice from,
    // I am misaligning the indices that I previously found. I'll need a vec of
    // start and stop points to use on the ORIGINAL input
    //
    // New Strategy: .get from Don't to Do on original, then .replace that in the accumulator
    // pruned = zipper.fold(
    //     String::from(string),
    //     |mut acc: String, ((start, _), (stop, _))| {
    //         if stop != 0 {
    //             // println!("Slicing from:\n {string}");
    //             let to_remove = string
    //                 .get(start..stop + 4)
    //                 .expect("Do list was longer than don't");
    //             // println!("{to_remove}");
    //             acc = acc.replace(to_remove, "");
    //         } else {
    //             acc = acc.get(..start).unwrap().to_string();
    //         }
    //         // println!("{acc}");
    //         acc
    //     },
    // );
    // }

    println!("Finding tuples in: {pruned}");

    // Find and collect the tuples
    pruned.split("mul(").fold(Vec::new(), |mut acc, seq| {
        let end_of_seq = seq.find(|c: char| !c.is_ascii_digit() && c != ',').unwrap();
        if seq.starts_with(|c: char| c.is_ascii_digit())
            && seq.chars().nth(end_of_seq).unwrap() == ')'
        {
            let (first, _) = seq.split_at(end_of_seq);
            let nums: Vec<u32> = first
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            // println!("{:?}", nums);
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

    #[test]
    fn dont_without_do() {
        let result = find_tuples("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+");
        assert_eq!(result, vec![(2, 4)]);
    }

    #[test]
    fn do_reenables_tuples() {
        let result = find_tuples(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn extra_donts_do_nothing() {
        let result = find_tuples(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](don't()don't()don't()don't()mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()...do()?mul(8,5))",
        );
        assert_eq!(result, vec![(2, 4), (8, 5), (2, 4), (8, 5)]);
    }

    #[test]
    fn multiple_do_and_dont() {
        let result = find_tuples(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()...do()?mul(8,5))",
        );
        assert_eq!(result, vec![(2, 4), (8, 5), (2, 4), (8, 5)]);
    }

    #[test]
    fn ignore_do_before_dont() {
        let result = find_tuples(
            "xmul(2,4)do()&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn only_count_do_after_dont() {
        let result = find_tuples(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64]do()mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()...do()?mul(8,5))",
        );
        assert_eq!(result, vec![(2, 4), (11, 8), (8, 5), (2, 4), (8, 5)]);
    }
}
