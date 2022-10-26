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

struct Position {
    x: u32,
    y: u32,
}

struct Sub {
    pos: Position,
    aim: u32,
}

impl Sub {
    fn mv(&mut self, dir: Direction) {
        match dir {
            Direction::Up(scalar) => self.aim -= scalar,
            Direction::Down(scalar) => self.aim += scalar,
            Direction::Forward(scalar) => {
                self.pos.x += scalar;
                self.pos.y += scalar * self.aim;
            }
        }
    }
}

enum Direction
{
    Up(u32),
    Down(u32),
    Forward(u32)
}

impl Direction {
    fn from(s: &str) -> Direction {
        let s: Vec<&str> = s.split_ascii_whitespace().collect();
        let scalar = s[1].parse::<u32>().unwrap();
        match s[0] {
            "down" => Direction::Down(scalar),
            "forward" => Direction::Forward(scalar),
            "up" => Direction::Up(scalar),
            _ => panic!("ahh")
        }
    }
}



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut sub = Sub {
        pos: Position {
            x: 0,
            y: 0
        },
        aim: 0
    };

    fs::read_to_string(config.file_path)
        .expect("Can't read input.")
        .lines()
        .for_each(|s| sub.mv(Direction::from(s)));

    println!("Answer {}", sub.pos.x * sub.pos.y);
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_with_numbers() {
        // numbers from website https://adventofcode.com/2021/day/2
        let content= "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2".to_string();
        assert_eq!(vec![("forward".to_string(), 5 as u32),
                        ("down".to_string(), 5 as u32),
                        ("forward".to_string(), 8 as u32),
                        ("up".to_string(), 3 as u32),
                        ("down".to_string(), 8 as u32),
                        ("forward".to_string(), 2 as u32)],
                   parse(content));
    }
}