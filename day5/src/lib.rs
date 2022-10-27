use std::error::Error;
use std::fmt::{Formatter};
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

struct VentMap {
    lines: Vec<Line>,
}

impl VentMap {
    fn builder(s: &str) -> VentMap {
        let mut v = VentMap { lines: vec![] };
        for line in s.lines() {
            v.lines.push(Line::builder(line));
        }
        v
    }

    fn get(&self) -> usize {
        let mut xmax = 0;
        let mut ymax = 0;
        for line in &self.lines {
            if line.start_point.x > xmax {
                xmax = line.start_point.x;
            }
            if line.end_point.x > xmax {
                xmax = line.end_point.x;
            }
            if line.start_point.y > ymax {
                ymax = line.start_point.y;
            }
            if line.end_point.y > ymax {
                ymax = line.end_point.y;
            }
        }
        let mut map = vec![vec![0 as usize; xmax + 1]; ymax + 1];
        for line in &self.lines {
            match line.direction() {
                None => {}
                Some(Direction::Up) => {
                    for step in 0..line.len() {
                        map[line.start_point.y - step][line.start_point.x] += 1;
                    }
                }
                Some(Direction::Down) => {
                    for step in 0..line.len() {
                        map[line.start_point.y + step][line.start_point.x] += 1;
                    }
                }
                Some(Direction::Left) => {
                    for step in 0..line.len() {
                        map[line.start_point.y][line.start_point.x - step] += 1;
                    }
                }
                Some(Direction::Right) => {
                    for step in 0..line.len() {
                        map[line.start_point.y][line.start_point.x + step] += 1;
                    }
                }
                Some(Direction::UpLeft) => {
                    for step in 0..line.len() {
                        map[line.start_point.y - step][line.start_point.x - step] += 1;
                    }
                }
                Some(Direction::UpRight) => {
                    for step in 0..line.len() {
                        map[line.start_point.y - step][line.start_point.x + step] += 1;
                    }
                }
                Some(Direction::DownLeft) => {
                    for step in 0..line.len() {
                        map[line.start_point.y + step][line.start_point.x - step] += 1;
                    }
                }
                Some(Direction::DownRight) => {
                    for step in 0..line.len() {
                        map[line.start_point.y + step][line.start_point.x + step] += 1;
                    }
                }
            }
        }
        map.iter().flatten().filter(|x| **x >= 2).count()
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(PartialEq, Debug)]
struct Line {
    start_point: Point,
    end_point: Point,

}

impl Line {
    fn builder(s: &str) -> Line {
        let (start, end) = s.split_once(" -> ").expect("not a line");
        Line { start_point: Point::builder(start), end_point: Point::builder(end) }
    }

    fn direction(&self) -> Option<Direction> {
        if self.start_point.x == self.end_point.x {
            if self.start_point.y < self.end_point.y {
                return Some(Direction::Down);
            } else if self.start_point.y > self.end_point.y {
                return Some(Direction::Up);
            }
        }
        if self.start_point.y == self.end_point.y {
            if self.start_point.x < self.end_point.x {
                return Some(Direction::Right);
            } else if self.start_point.x > self.end_point.x {
                return Some(Direction::Left);
            }
        }
        if self.start_point.x.abs_diff(self.end_point.x) == self.start_point.y.abs_diff(self.end_point.y) {
            if self.start_point.x > self.end_point.x { // Left
                if self.start_point.y > self.end_point.y { // Up
                    return Some(Direction::UpLeft);
                } else if self.start_point.y < self.end_point.y { //Down
                    return Some(Direction::DownLeft);
                }
            } else if self.start_point.x < self.end_point.x { //Right
                if self.start_point.y > self.end_point.y { // Up
                    return Some(Direction::UpRight);
                } else if self.start_point.y < self.end_point.y { //Down
                    return Some(Direction::DownRight);
                }
            }
        }

        None
    }

    fn len(&self) -> usize {
        match self.direction() {
            None => { 0 }
            Some(Direction::Up) => {
                self.start_point.y - self.end_point.y + 1
            }
            Some(Direction::Down) => {
                self.end_point.y - self.start_point.y + 1
            }
            Some(Direction::Left) => {
                self.start_point.x - self.end_point.x + 1
            }
            Some(Direction::Right) => {
                self.end_point.x - self.start_point.x + 1
            }
            Some(Direction::UpLeft) => {
                self.start_point.x - self.end_point.x + 1
            }
            Some(Direction::UpRight) => {
                self.end_point.x - self.start_point.x + 1
            }
            Some(Direction::DownLeft) => {
                self.start_point.x - self.end_point.x + 1
            }
            Some(Direction::DownRight) => {
                self.end_point.x - self.start_point.x + 1
            }
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{} -> {},{}", self.start_point.x, self.start_point.y, self.end_point.x, self.end_point.y)
    }
}

#[derive(PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn builder(s: &str) -> Point {
        let (n1, n2) = s.split_once(',').expect("not comma separated");
        Point { x: n1.parse().expect("not a number"), y: n2.parse().expect("not a number") }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let content = fs::read_to_string(config.file_path)?.replace("\r\n", "\n");
    let map = VentMap::builder(content.as_str());
    println!("Danger {}", map.get());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vent_map_builder() {
        let content = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let map = VentMap::builder(content);
        assert_eq!(Line {
            start_point: Point { x: 9, y: 4 },
            end_point: Point { x: 3, y: 4 },
        }, map.lines[2]);
        assert_eq!(10, map.lines.len());
    }

    #[test]
    fn test_vent_map_get() {
        let content = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let map = VentMap::builder(content);
        assert_eq!(12, map.get());
    }

    #[test]
    fn test_line_builder() {
        let content = "3,4 -> 1,4";
        let line = Line::builder(content);
        assert_eq!(Point { x: 3, y: 4 }, line.start_point);
        assert_eq!(Point { x: 1, y: 4 }, line.end_point);
    }

    #[test]
    fn test_line_direction() {
        let content = "3,4 -> 1,4";
        let line = Line::builder(content);
        assert_eq!(Some(Direction::Left), line.direction());
        let content = "8,0 -> 0,8";
        let line = Line::builder(content);
        assert_eq!(Some(Direction::DownLeft), line.direction());
        let content = "7,0 -> 7,4";
        let line = Line::builder(content);
        assert_eq!(Some(Direction::Down), line.direction());
        let content = "6,4 -> 2,0";
        let line = Line::builder(content);
        assert_eq!(Some(Direction::UpLeft), line.direction());
        let content = "0,0 -> 2,3";
        let line = Line::builder(content);
        assert_eq!(None, line.direction());
    }

    #[test]
    fn test_line_len() {
        let content = "3,4 -> 1,4";
        let line = Line::builder(content);
        assert_eq!(3, line.len())
    }

    #[test]
    fn test_point_builder() {
        let content = "3,4";
        let point = Point::builder(content);
        assert_eq!(Point { x: 3, y: 4 }, point);
    }
}
