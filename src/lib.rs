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

    let similarity: u32 = first
        .iter()
        .map(|x| {
            let mut count = 0;
            for num in &second {
                if num == x {
                    count += 1;
                }
            }
            x * count
        })
        .sum();

    let zipped = first.iter().zip(second.iter());

    let distance: u32 = zipped.map(|(a, b)| a.abs_diff(*b)).sum();

    ListStats {
        distance,
        similarity,
    }
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
        }
        day => println!("No logic for day {day}"),
    }

    Ok(())
}
