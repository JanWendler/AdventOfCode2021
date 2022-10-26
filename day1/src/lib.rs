use std::error::Error;
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let numbers = string_to_num_vec(content);
    println!("Number of greater values: {}", number_of_greater_values(sum_three(numbers)));
    Ok(())
}

pub fn string_to_num_vec(content: String) -> Vec<u32> {
    content
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

pub fn number_of_greater_values(numbers: Vec<u32>) -> usize {
    // adapted solution from u/u/-WorstWizard
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

pub fn sum_three(numbers: Vec<u32>) -> Vec<u32> {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .zip(numbers.iter().skip(2))
        .map(|((a,b),c)| a+b+c)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_greater_values() {
        // numbers from website https://adventofcode.com/2021/day/1
        let content= vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(7, number_of_greater_values(content));
    }

    #[test]
    fn sum_three_values() {
        let numbers = vec![2,3,4,5,6];
        assert_eq!(vec![9,12,15], sum_three(numbers));
    }
}