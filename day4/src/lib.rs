use std::error::Error;
use std::fs;
use std::ops::Sub;

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

struct Submarine {
    bingo: Bingo,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine { bingo: Bingo::new() }
    }
}

struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
    winning_number: u32,
    winning_board: Board,
}

impl Bingo {
    fn new() -> Bingo {
        Bingo {
            numbers: vec![],
            boards: vec![],
            winning_number: 0,
            winning_board: Board::new(),
        }
    }
    fn builder(s: &str) -> Bingo {
        let mut bingo = Bingo::new();
        let (numbers, boards) = s.split_once("\r\n\r\n").expect("no newline");
        bingo.numbers = numbers.split(",").map(|x| x.parse().expect("")).collect::<Vec<_>>();
        for board in boards.split("\r\n\r\n") {
            bingo.boards.push(Board::builder(board));
        }
        bingo
    }
    fn from(&mut self, s: &str) -> Result<&mut Bingo, Box<dyn Error>> {
        let (numbers, boards) = s.split_once("\r\n\r\n").expect("no newline");
        self.numbers = numbers.split(",").map(|x| x.parse().expect("")).collect::<Vec<_>>();
        for board in boards.split("\r\n\r\n") {
            self.boards.push(Board::builder(board));
        }
        Ok(self)
    }

    fn play(&mut self) -> Result<&Bingo, &str> {
        for number in self.numbers.iter() {
            for board in self.boards.iter_mut() {
                board.mark(number);
                if board.check() {
                    self.winning_number = *number;
                    self.winning_board = *board;
                    return Ok(self);
                }
            }
        }
        Err("could not find a winning board")
    }

    fn get(&self) -> u32 {
        let mut result = 0;
        for field in self.winning_board.fields.iter().flat_map(|r| r.iter()) {
            if field.drawn == false {
                result += field.value;
            }
        }
        result * self.winning_number
    }
}

#[derive(Copy)]
#[derive(Clone)]
struct Board {
    fields: [[Field; 5]; 5],
}

impl Board {
    fn new() -> Board {
        Board { fields: [[Field::new(); 5]; 5] }
    }
    fn builder(s: &str) -> Board {
        let mut board = Board::new();
        for (i, line) in s.lines().enumerate() {
            for (j, number) in line.split_whitespace().filter_map(|x| x.parse().ok()).enumerate() {
                board.fields[i][j].value = number;
                board.fields[i][j].drawn = false;
            }
        }
        board
    }
    fn mark(&mut self, number: &u32) -> &Board {
        for field in self.fields.iter_mut().flat_map(|r| r.iter_mut()) {
            if field.value == *number {
                field.drawn = true;
                break;
            }
        }
        self
    }
    fn check(&self) -> bool {
        for row in self.fields.iter() {
            for (i, field) in row.iter().enumerate() {
                if field.drawn {
                    if i >= 4 {
                        return true;
                    }
                    continue;
                }
                break;
            }
        }
        false
    }
}

#[derive(Copy)]
#[derive(Clone)]
struct Field {
    value: u32,
    drawn: bool,
}

impl Field {
    fn new() -> Field {
        Field { value: 0, drawn: false }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let content = fs::read_to_string(config.file_path)?;
    let mut sub = Submarine::new();
    sub.bingo = Bingo::builder(content.as_str());
    println!("Answer {}", sub.bingo.play()?.get());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_play() {
        let content: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let mut bingo: Bingo = Bingo {
            numbers: vec![],
            boards: vec![],
            winning_number: 0,
            winning_board: Board {
                fields: [[Field { value: 0, drawn: false }; 5]; 5]
            },
        };
        bingo.from(content).expect("REASON").play().expect("REASON");
        for board in bingo.boards {
            for (i, row) in board.fields.iter().enumerate() {
                for (j, field) in row.iter().enumerate() {
                    print!("{}/{}/={},{} ", i, j, field.value, field.drawn);
                }
                println!();
            }
        }
        assert_eq!(24, bingo.winning_number);
        assert_eq!(24, bingo.winning_board.fields[0][3].value);
    }

    #[test]
    fn test_board_mark() {
        let mut board: Board = Board::builder("22 13 17 11  0
                                          8  2 23  4 24
                                          21  9 14 16  7
                                          6 10  3 18  5
                                          1 12 20 15 19");
        assert_eq!(false, board.fields[0][0].drawn);
        board.mark(&22);
        assert_eq!(true, board.fields[0][0].drawn);
    }

    #[test]
    fn test_board_check() {
        let mut board: Board = Board::builder("22 13 17 11  0
                                          8  2 23  4 24
                                          21  9 14 16  7
                                          6 10  3 18  5
                                          1 12 20 15 19");
        assert_eq!(false, board.check());
        for i in 0..5 {
            board.fields[1][i].drawn = true;
        }
        assert_eq!(true, board.check());
    }

    #[test]
    fn test_bingo_get()
    {
        let bingo: Bingo = Bingo {
            numbers: vec![],
            boards: vec![],
            winning_number: 24,
            winning_board: Board {
                fields: [
                    [Field { value: 14, drawn: true }, Field { value: 21, drawn: true }, Field { value: 17, drawn: true }, Field { value: 24, drawn: true }, Field { value: 4, drawn: true }],
                    [Field { value: 10, drawn: false }, Field { value: 16, drawn: false }, Field { value: 15, drawn: false }, Field { value: 9, drawn: true }, Field { value: 19, drawn: false }],
                    [Field { value: 18, drawn: false }, Field { value: 8, drawn: false }, Field { value: 23, drawn: true }, Field { value: 26, drawn: false }, Field { value: 20, drawn: false }],
                    [Field { value: 22, drawn: false }, Field { value: 11, drawn: true }, Field { value: 13, drawn: false }, Field { value: 6, drawn: false }, Field { value: 5, drawn: true }],
                    [Field { value: 2, drawn: true }, Field { value: 0, drawn: true }, Field { value: 12, drawn: false }, Field { value: 3, drawn: false }, Field { value: 7, drawn: true }], ]
            },
        };

        assert_eq!(4512, bingo.get());
    }

    #[test]
    fn test_bingo_from()
    {
        let content: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let mut bingo: Bingo = Bingo {
            numbers: vec![],
            boards: vec![],
            winning_number: 0,
            winning_board: Board {
                fields: [[Field { value: 0, drawn: false }; 5]; 5]
            },
        };
        assert_eq!(18, bingo.from(content).expect("").boards.get(1).expect("").fields[1][1].value);
    }

    #[test]
    fn test_board_from() {
        let content = "14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  1 12  3  7";
        let board: Board = Board::builder(content);
        for (i, row) in board.fields.iter().enumerate() {
            for (j, field) in row.iter().enumerate() {
                print!("{}/{}/={} ", i, j, field.value);
            }
            println!();
        }
        assert_eq!(16, board.fields[1][1].value);
        assert_eq!(21, board.fields[0][1].value);
        assert_eq!(3, board.fields[4][3].value);
    }
}
