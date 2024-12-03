use std::error::Error;
use std::fs;

pub struct Config {
    pub day: u8,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // We don't need the name of the program

        let day = match args.next() {
            Some(arg) => arg.parse::<u8>().expect("Day must be a number"),
            None => return Err("Didn't get a day"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config { day, file_path })
    }
}

struct ListStats {
    distance: u32,
    similarity: u32,
}

fn day_1<'a>(lines: impl Iterator<Item = &'a str>) -> ListStats {
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

fn is_safe(list: &str) -> bool {
    true
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Split the file into an iterator containing all the lines of the file;
    let lines = contents.split_terminator('\n');

    match config.day {
        1 => {
            let stats = day_1(lines);

            println!("Distance: {}", stats.distance);
            println!("Similarity: {}", stats.similarity);
        }
        2 => {
            println!("Let's hear it for day 2!");
            let total_safe = lines.fold(0, |acc, line| acc + is_safe(line) as u8);

            println!("There were {total_safe} safe lines");
        }
        day => println!("No logic for day {day}"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_line() {
        let result = is_safe("7 6 4 2 1");
        assert_eq!(true, result);
    }

    #[test]
    fn increases_too_quickly() {
        let result = is_safe("1 2 7 8 9");
        assert_eq!(false, result);
    }

    #[test]
    fn decreases_too_quickly() {
        let result = is_safe("9 7 6 2 1");
        assert_eq!(false, result);
    }

    #[test]
    fn direction_change() {
        let result = is_safe("1 3 2 4 5");
        assert_eq!(false, result);
    }

    #[test]
    fn no_direction() {
        let result = is_safe("8 6 4 4 1");
        assert_eq!(false, result);
    }

    #[test]
    fn increase_by_3() {
        let result = is_safe("1 3 6 7 9");
        assert_eq!(true, result);
    }
}
