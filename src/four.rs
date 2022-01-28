use super::filez;

struct Board {
    rows: [[u8; 5]; 5],
    checked: [[bool; 5]; 5],
    done: bool,
}

impl Board {

    fn check_number(&mut self, n: u8) {
        if self.done {
            return;
        }
        for r in 0..5 {
            for c in 0..5 {
                if self.rows[r][c] == n {
                    self.checked[r][c] = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        if self.done {
            return false;
        }
        // by row
        for r in 0..5 {
            let mut winner = true;
            for c in 0..5 {
                winner = winner & self.checked[r][c];
            }
            if winner {
                return true;
            }
        }
        // by col
        for c in 0..5 {
            let mut winner = true;
            for r in 0..5 {
                winner = winner & self.checked[r][c];
            }
            if winner {
                return true;
            }
        }
        return false;
    }

    fn score(&mut self, picked: u64) -> u64 {
        let mut score: u64 = 0;

        for r in 0..5 {
            for c in 0..5 {
                if !self.checked[r][c] {
                    score += self.rows[r][c] as u64;
                }
            }
        }
        self.done = true;
        return score * picked;
    }
}


pub fn part1() {
    // println!("BINGO is starting");
    let v = load_data_4();
    let numbers = v.0;
    let mut boards = v.1;

    // println!("Will go through {} numbers...", numbers.len());
    for picked in numbers {
        // println!("Picked number {}", picked);
        for b in boards.iter_mut() {
            b.check_number(picked);
            if b.is_winner() {
                println!("04.1 winner score: {}", b.score(picked as u64));
                return;
            }
        }
    }
}

pub fn part2() {
    // println!("BINGO (loser) is starting");
    let v = load_data_4();
    let numbers = v.0;
    let mut boards = v.1;
    let mut score: u64 = 0;

    // println!("Will go through {} numbers...", numbers.len());
    for picked in numbers {
        // println!("Picked number {}", picked);
        let boards_len = boards.len();
        for i in 0..boards_len {
            let b = &mut boards[i];
            b.check_number(picked);
            if b.is_winner() {
                score = b.score(picked as u64);
            }
        }
    }
    println!("04.2 loser score: {}", score);
}

fn load_data_4() -> (Vec<u8>, Vec<Board>) {
    let mut numbers = Vec::<u8>::new();
    let mut boards = Vec::<Board>::new();

    if let Ok(lines) = filez::read_lines("inputs/04.txt") {
        let mut board_rows_raw = Vec::<String>::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(raw) = line {
                
                if numbers.len() == 0 {
                    // first line is inputs
                    let elems: Vec<&str> = raw.split(',').collect();
                    for nz in elems {
                        let parsed: u8 = nz.trim().parse().unwrap();
                        numbers.push(parsed);
                    }
                } else {
                    // build a board!
                    if raw == "" {
                        boards.push(new_board(board_rows_raw.to_owned()));
                        board_rows_raw.clear();
                        continue;
                    }
                    board_rows_raw.push(raw.to_owned());
                }
            }
        }
    }


    return (numbers, boards);
}

fn new_board(rows: Vec<String>) -> Board {
    return Board {
        rows: build_rows(rows),
        checked: unchecked_rows(),
        done: false,
    };
}

fn build_rows(raw_rows: Vec<String>) -> [[u8; 5]; 5] {
    let mut u8_rows: [[u8; 5]; 5] = [[0; 5]; 5];

    for i in 0..raw_rows.len() {
        let elems: Vec<&str> = raw_rows[i].split_whitespace().collect::<Vec<&str>>();
        let n = [
            elems[0].parse::<u8>().unwrap(),
            elems[1].parse::<u8>().unwrap(),
            elems[2].parse::<u8>().unwrap(),
            elems[3].parse::<u8>().unwrap(),
            elems[4].parse::<u8>().unwrap(),
        ];
        u8_rows[i] = n;
    }

    return u8_rows;
}

fn unchecked_rows() -> [[bool; 5]; 5] {
    return [[false; 5]; 5];
}
