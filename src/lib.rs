use std::error::Error;
use std::fs;

pub struct Config {
    pub day: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // We don't need the name of the program

        let day = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a day"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config { day, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");

    // Split into two vectors
    let iter = contents.split('\n');
    let mut first: Vec<u32> = [].to_vec();
    let mut second: Vec<u32> = [].to_vec();

    for line in iter {
        // println! {"{line}"};

        if line != "" {
            let mut words = line.split_whitespace();
            match words.next() {
                Some(word) => first.push(word.parse::<u32>().unwrap()),
                None => return Err("There's nothing here".into()),
            }
            match words.next() {
                Some(word) => second.push(word.parse::<u32>().unwrap()),
                None => return Err("There's nothing here".into()),
            }
        }
    }

    first.sort();
    second.sort();

    let cloned = first.clone();
    let also_cloned = second.clone();

    // for number in cloned {
    //     println!("{number}");
    //     let mut count = 0;
    //     for spot in &also_cloned {
    //         if *spot == number {
    //             count += 1;
    //         }
    //     }
    //     println!("Matched {count} times");
    // }

    let similarity: u32 = cloned
        .iter()
        .map(|x| {
            let mut count = 0;
            for num in &also_cloned {
                if num == x {
                    count += 1;
                }
            }
            x * count
        })
        .sum();

    println!("Similarity: {similarity}");

    let zipped = first.iter().zip(second.iter());

    let distance: u32 = zipped.map(|(a, b)| a.abs_diff(*b)).sum();

    println!("Distance: {distance}");

    Ok(())
}
