use std::error::Error;
use std::fs;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Split the file into an iterator containing all the lines of the file;
    let lines = contents.split_terminator('\n');

    match config.day {
        1 => {
            let stats = day_1::get_stats(lines);

            println!("Distance: {}", stats.distance);
            println!("Similarity: {}", stats.similarity);
        }
        2 => {
            let total_safe = lines.fold(0, |acc, line| {
                let vec = day_2::parse_list(line);
                acc + (day_2::is_safe(&vec) || day_2::dampened(&vec)) as u32
            });

            println!("There were {total_safe} safe lines");
        }
        3 => {
            let total = day_3::sum_of_multiplied(day_3::find_tuples(&contents));

            println!("The total is {total}");
        }
        4 => {
            let xmas = day_4::find_xmas(day_4::enter_the_matrix(lines.clone()));
            let cross_mas = day_4::find_cross_mas(day_4::enter_the_matrix(lines));

            println!("Total XMAS: {xmas}");
            println!("Total X-MAS: {cross_mas}");
        }
        5 => {
            let (rules, pages) = day_5::split_rules_and_pages(&contents);
            let checked_lists = day_5::find_good_lists(&rules, pages);
            let sum_of_mid = checked_lists[&String::from("correct")]
                .iter()
                .fold(0, |acc, list| acc + list[list.len() / 2]);
            let sum_of_fixed =
                day_5::fix_lists(&rules, checked_lists[&String::from("incorrect")].clone())
                    .iter()
                    .fold(0, |acc, list| acc + list[list.len() / 2]);

            println!("Sum of the middle pages in correct lists is {sum_of_mid}");
            println!("Sum of the middle pages in fixed lists is {sum_of_fixed}");
        }
        6 => {
            println!("Solving puzzles is as good as a pick six!")
        }
        day => println!("No logic for day {day}"),
    }

    Ok(())
}
