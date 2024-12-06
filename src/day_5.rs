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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parses_properly() {
        let rules = HashMap::from([
            (
                47,
                Rules {
                    before: vec![53],
                    after: vec![],
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
                    before: vec![13],
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
        let pages = vec![vec![47, 53, 97, 13], vec![97, 53, 47, 13]];

        let result = split_rules_and_pages("47|53\n97|13\n\n47,53,97,13\n97,53,47,13");
        assert_eq!(result, (rules, pages))
    }
}
