use std::error::Error;
use std::fs;

fn binstr_to_dec(bin: &str) -> Result<u32, Box<dyn Error>> {
    Ok(u32::from_str_radix(bin, 2)?)
}

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

struct Report {
    pc: PowerConsumption,
    lsr: LifeSupportRating,
}

impl Report {}

struct PowerConsumption
{
    gamma_rate: u32,
    epsilon_rate: u32,
}

impl PowerConsumption {
    fn from(&mut self, s: &str) -> &PowerConsumption {
        let lines: Vec<&str> = s.split("\n").collect();
        let mut count: [i32; 12] = [0; 12];
        for line in lines {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '0' => count[i] -= 1,
                    '1' => count[i] += 1,
                    _ => continue
                };
            }
        }
        let mut g = "".to_string();
        for x in count {
            if x < 0 {
                g.push('0');
            } else if x > 0 {
                g.push('1');
            }
        }
        let e: String =
            g
                .chars()
                .map(|c| match c {
                    '0' => '1',
                    _ => '0',
                })
                .collect();
        self.gamma_rate = binstr_to_dec(&g).expect("Not a binary number!");
        self.epsilon_rate = binstr_to_dec(&e).expect("Not a binary number!");
        self
    }

    fn get(&self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

struct LifeSupportRating
{
    co2_rating: u32,
    o_gen_rating: u32,
}

impl LifeSupportRating {
    fn from(&mut self, s: &str, bits: u32) -> &LifeSupportRating {
        let lines: Vec<u32> = s.split("\r\n").map(|x| binstr_to_dec(x).expect("")).collect::<Vec<_>>();
        let base: u32 = 2;
        let mut most = lines.clone();
        for i in 0..bits {
            let mut count: i32 = 0;
            for line in &most {
                if (*line & base.pow(bits-1-i)) > 0 {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            if count >= 0 {
                most = most.into_iter().filter(|n| (*n & base.pow(bits-1-i)) > 0).collect::<Vec<_>>();
            } else {
                most = most.into_iter().filter(|n| (*n & base.pow(bits-1-i)) == 0).collect::<Vec<_>>();
            }
            if most.len() == 1 {
                break;
            }
        }
        self.co2_rating = *most.first().expect("no number");

        let mut least = lines.clone();
        for i in 0..bits {
            let mut count: i32 = 0;
            for line in &least {
                if (*line & base.pow(bits-1-i)) > 0 {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            if count >= 0 {
                least = least.into_iter().filter(|n| (*n & base.pow(bits-1-i)) == 0).collect::<Vec<_>>();
            } else {
                least = least.into_iter().filter(|n| (*n & base.pow(bits-1-i)) > 0).collect::<Vec<_>>();
            }
            if least.len() == 1 {
                break;
            }
        }
        self.o_gen_rating = *least.first().expect("no number");
        println!("{}/{}",self.o_gen_rating, self.co2_rating);
        self
    }

    fn get(&self) -> u32 {
        self.o_gen_rating * self.co2_rating
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let mut rep = Report
    {
        pc: PowerConsumption
        {
            gamma_rate: 0,
            epsilon_rate: 0,
        },
        lsr: LifeSupportRating
        {
            co2_rating: 0,
            o_gen_rating: 0,
        },
    };
    let content = fs::read_to_string(config.file_path)?;

    println!("Answer {}", rep.pc.from(&content).get());
    println!("Answer {}", rep.lsr.from(&content,12).get());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_with_numbers() {
        let content: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let mut lsr = LifeSupportRating{
            co2_rating: 0,
            o_gen_rating: 0
        };
        assert_eq!(230, lsr.from(content,5).get());
    }
}