use std::error::Error;
use std::fmt::Formatter;
use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item=String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config {
            file_path,
        })
    }
}

struct School {
    lanternfish: Vec<usize>,
}

impl School {
    fn new() -> School {
        School {
            lanternfish: vec![]
        }
    }
    fn from_str(s: &str) -> School {
        let mut school = School::new();
        for fish in s.split(',') {
            school.lanternfish.push(fish.parse::<usize>().expect(""));
        }
        school
    }
    fn next_day(&mut self) -> &mut School {
        let mut count = 0;
        for fish in &mut self.lanternfish {
            if *fish == 0 {
                count += 1;
                *fish = 6;
            }
            else{
                *fish -= 1;
            }

        }
        for _ in 0..count {
            self.lanternfish.push(8);
        }
        self
    }
    fn after_n_days(&mut self, days: usize) -> usize {
        for _ in 0..days {
            self.next_day();
        }
        self.lanternfish.len()
    }
}

impl std::fmt::Display for School {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.lanternfish)
    }
}
#[inline]
fn simulate_day(day: usize, fishes: &mut [u128]) {
    fishes[(day + 7) % 9] += fishes[day % 9];
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{

    let mut fishes = [0u128; 9];
    let content = fs::read_to_string(config.file_path)?;
    // parse the input: each case represents the number of fishes with the case index as timer
    for fish in content.trim().split(",").map(|n| n.parse::<usize>().expect("Fish timer must be an integer")) {
        fishes[fish] += 1;
    }

    // simulation
    for day in 0..80 {
        simulate_day(day, &mut fishes);
    }
    println!("{}", fishes.iter().sum::<u128>());
    for day in 80..256 {
        simulate_day(day, &mut fishes);
    }
    println!("{}", fishes.iter().sum::<u128>());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_school_after_n_days() {
        let content = "3,4,3,1,2";
        let days = 18;
        let mut school = School::from_str(content);
        assert_eq!(26, school.after_n_days(days));
        let mut school = School::from_str(content);
        let days = 80;
        assert_eq!(5934, school.after_n_days(days));
    }

    #[test]
    fn test_school_from_str() {
        let content = "3,4,3,1,2";
        let school = School::from_str(content);
        assert_eq!(5, school.lanternfish.len());
        assert_eq!(1, school.lanternfish[3]);
    }

    #[test]
    fn test_school_next_day() {
        let content = "3,4,3,1,2";
        let mut school = School::from_str(content);
        assert_eq!(5, school.next_day().lanternfish.len());
        assert_eq!(6, school.next_day().lanternfish.len());
        assert_eq!(7, school.next_day().lanternfish.len());
    }
}
