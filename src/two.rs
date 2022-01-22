use super::filez;

enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct AimedPosition {
    x: i64,
    y: i64,
    aim: i64,
}

struct Command {
    direction: Direction,
    value: i64,
}

impl Command {
    fn apply_to(&self, pos: &mut Position) {
        match &self.direction {
            Direction::Up => {
                pos.y -= self.value;
            }
            Direction::Down => {
                pos.y += self.value;
            }
            Direction::Forward => {
                pos.x += self.value;
            }
        }
    }

    fn apply_with_aim_to(&self, pos: &mut AimedPosition) {
        match &self.direction {
            Direction::Up => {
                pos.aim -= self.value;
            }
            Direction::Down => {
                pos.aim += self.value;
            }
            Direction::Forward => {
                pos.x += self.value;
                pos.y += self.value * pos.aim;
            }
        }
    }
}

impl Direction {
    fn from_string(v: &str) -> Direction {
        if v == "up" {
            return Direction::Up;
        }

        if v == "down" {
            return Direction::Down;
        }

        if v == "forward" {
            return Direction::Forward;
        }

        panic!("Invalid direction {}", v);
    }
}

fn load_data_2(file_name: &str) -> Vec<Command> {
    let mut input = Vec::new();
    if let Ok(lines) = filez::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                if num == "" {
                    continue;
                }
                let elems: Vec<&str> = num.split(' ').collect();
                let d = Direction::from_string( elems[0] );
                let a = match elems[1].parse() {
                    Ok(num) => num,
                    Err(x) => panic!("BAD {}", x),
                };

                input.push(Command{
                    direction: d,
                    value: a,
                });
            }
        }
    }
    return input;
}

pub fn part1() {
    let commands = load_data_2("inputs/02.txt");
    let mut pos = Position{x:0, y:0};
    for c in commands {
        c.apply_to(&mut pos);
    }

    println!("02.1 coordinates and score: {:?} = {}", pos, pos.x * pos.y);
}

pub fn part2() {
    let commands = load_data_2("inputs/02.txt");
    let mut pos = AimedPosition{x:0, y:0, aim:0};
    for c in commands {
        c.apply_with_aim_to(&mut pos);
    }

    println!("02.2 coordinates and score: {:?} = {}", pos, pos.x * pos.y);
}
