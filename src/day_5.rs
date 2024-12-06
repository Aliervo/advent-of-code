// Parse the data
// Filter the list of lists such that only lists which obey all rules remain -> use .all(|e|)
//  -- .split matching the value, giving a before and after iter
//    -- .split(|p| p == e) -> iter.next().unwrap() x2
//  -- .all both of those to see if the respective rule .contains them
//    -- above assumes non-empty
//    -- before.all(|n| rules[e].after.contains(n)) && after.all(...
// Fold result starting at 0 finding the middle number with .len() / 2 and adding
//  -- result.fold(0, |acc, vec| acc + vec[vec.len() / 2])

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Rules {
    before: Vec<u16>,
    after: Vec<u16>,
}

pub fn split_rules_and_pages(string: &str) -> (HashMap<u16, Rules>, Vec<Vec<u16>>) {
    let mut iter = string.split("\n\n");
    let rules = iter
        .next()
        .unwrap()
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, rule| {
            let pair: Vec<u16> = rule.split('|').map(|x| x.parse::<u16>().unwrap()).collect();
            let first = acc.entry(pair[0]).or_insert(Rules {
                before: vec![pair[1]],
                after: Vec::new(),
            });
            if !first.before.contains(&pair[1]) {
                first.before.push(pair[1])
            }
            let second = acc.entry(pair[1]).or_insert(Rules {
                before: Vec::new(),
                after: vec![pair[0]],
            });
            if !second.after.contains(&pair[0]) {
                second.after.push(pair[0])
            }
            acc
        });
    println!("{:?}", rules);

    let pages: Vec<Vec<u16>> = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.split(',').map(|n| n.parse::<u16>().unwrap()).collect())
        .collect();
    println!("{:?}", pages);

    (rules, pages)
}

fn find_good_lists(rules: HashMap<u16, Rules>, list_of_lists: Vec<Vec<u16>>) -> Vec<Vec<u16>> {
    vec![vec![0]]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_data() -> (HashMap<u16, Rules>, Vec<Vec<u16>>) {
        let test_rules = HashMap::from([
            (
                47,
                Rules {
                    before: vec![53],
                    after: vec![97],
                },
            ),
            (
                53,
                Rules {
                    before: vec![],
                    after: vec![47],
                },
            ),
            (
                97,
                Rules {
                    before: vec![13, 47],
                    after: vec![],
                },
            ),
            (
                13,
                Rules {
                    before: vec![],
                    after: vec![97],
                },
            ),
        ]);
        let test_pages = vec![vec![97, 47, 53], vec![97, 53, 47]];

        (test_rules, test_pages)
    }

    #[test]
    fn input_parses_properly() {
        let result = split_rules_and_pages("47|53\n97|13\n97|47\n\n97,47,53\n97,53,47");
        assert_eq!(result, mock_data());
    }

    #[test]
    fn incorrect_lists_are_removed() {
        let (rules, pages) = mock_data();
        let result = find_good_lists(rules, pages);
        assert_eq!(result, vec![vec![97, 47, 53]]);
    }
}
