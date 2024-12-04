pub struct ListStats {
    pub distance: u32,
    pub similarity: u32,
}

pub fn get_stats<'a>(lines: impl Iterator<Item = &'a str>) -> ListStats {
    // Split into two vectors
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();

    for line in lines {
        let mut words = line.split_whitespace();
        first.push(
            words
                .next()
                .expect("The line was blank")
                .parse::<u32>()
                .expect("That wasn't a number"),
        );
        second.push(
            words
                .next()
                .expect("The line was blank")
                .parse::<u32>()
                .expect("That wasn't a number"),
        );
    }

    first.sort();
    second.sort();

    let similarity: u32 = first.iter().fold(0, |acc, x| {
        let mut count = 0;
        for num in &second {
            if num == x {
                count += 1;
            }
        }
        x * count + acc
    });

    let zipped = first.iter().zip(second.iter());

    let distance: u32 = zipped.fold(0, |acc, (a, b)| acc + a.abs_diff(*b));

    ListStats {
        distance,
        similarity,
    }
}
