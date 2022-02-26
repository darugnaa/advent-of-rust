use std::collections::HashSet;

use super::filez;

struct Board {
    r_state: usize,
    c_state: usize,
    data: Vec<Vec<u8>>,
}

impl Board {
    fn is_low_point(&self, row: usize, col: usize) -> bool {
        let value = self.data[row][col];

        // For each row and col coordinates, if those are on the border of the board
        // return MAX_VALUE for u8 so the comparisons will return TRUE. This means
        // only values in the board will be used to determine if point is a local minimum.
        let up_value = if row == 0 {
            255
        } else {
            self.data[row-1][col]
        };

        let down_value = if row == self.data.len() - 1 {
            255
        } else {
            self.data[row+1][col]
        };

        let left_value = if col == 0 {
            255
        } else {
            self.data[row][col-1]
        };

        let right_value = if col == self.data[row].len() - 1 {
            255
        } else {
            self.data[row][col+1]
        };

        return value < up_value &&
            value < down_value &&
            value < right_value &&
            value < left_value;
    }

    fn is_basin_candidate(&self, row: usize, col: usize) -> bool {
        if row >= self.data.len() {
            return false;
        }
        if col >= self.data[row].len() {
            return false;
        }
        return self.data[row][col] < 9;
    }

    fn basin_size(&self, point :LowPoint) -> usize {
        let mut basin = HashSet::<LowPoint>::new();
        let mut to_analyze = Vec::<LowPoint>::new();
        to_analyze.push(point);

        while !to_analyze.is_empty() {
            // Safe to unwrap because the while loop guarantees that there is
            // at least one element in the vector
            let p = to_analyze.pop().unwrap();
            // println!("Analyzing {:?}", p);
            basin.insert(p);
            
            // check up element only if there is one
            if p.row > 0 {
                let up_row = p.row - 1;
                let up_col = p.col;
                if self.is_basin_candidate(up_row, up_col) {
                    let basin_point = LowPoint{
                        row: up_row,
                        col: up_col,
                        value: self.data[up_row][up_col]
                    };
                    if !basin.contains(&basin_point) {
                       to_analyze.push(basin_point);
                    }
                }
            }
            if p.row + 1 < self.data.len() {
                let down_row = p.row + 1;
                let down_col = p.col;
                if self.is_basin_candidate(down_row, down_col) {
                    let basin_point = LowPoint{
                        row: down_row,
                        col: down_col,
                        value: self.data[down_row][down_col]
                    };
                    if !basin.contains(&basin_point) {
                        to_analyze.push(basin_point);
                    }
                }
            }
            if p.col > 0 {
                let left_row = p.row;
                let left_col = p.col - 1;
                if self.is_basin_candidate(left_row, left_col) {
                    let basin_point = LowPoint{
                        row: left_row,
                        col: left_col,
                        value: self.data[left_row][left_col]
                    };
                    if !basin.contains(&basin_point) {
                        to_analyze.push(basin_point);
                    }
                }
            }
            if p.col + 1 < self.data[p.row].len() {
                let right_row = p.row;
                let right_col = p.col + 1;
                if self.is_basin_candidate(right_row, right_col) {
                    let basin_point = LowPoint{
                        row: right_row,
                        col: right_col,
                        value: self.data[right_row][right_col]
                    };
                    if !basin.contains(&basin_point) {
                        to_analyze.push(basin_point);
                    }
                }
            }
        }

        return basin.len()
    }
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Copy,Clone)]
struct LowPoint {
    row: usize,
    col: usize,
    value: u8,
}

impl LowPoint {
    fn risk_level(&self) -> u8 {
        return self.value + 1;
    }
}

// Implementing an iterator over the board that returns the "low points".
// It is nested for loops for rows and columns, and state is stored between each call.
impl Iterator for Board {
    type Item = LowPoint;

    fn next(&mut self) -> Option<LowPoint> {
        let mut r = self.r_state;
        let mut c = self.c_state;
        // println!("Next iterator state {},{}", self.r_state, self.c_state);
        // Iterating using while conditions, as c-style for is not available in rust.
        // using .take_while approach become cumbersome as only the "first" loop here
        // must restart from stored iterator state.
        while r < self.data.len() {
            while c < self.data[r].len() {
                if self.is_low_point(r, c) {
                    // Store the next iterator state
                    self.r_state = r; // same row
                    self.c_state = c + 1; // next column
                    // Return current element of the iterator
                    return Some(LowPoint{
                        row: r,
                        col: c,
                        value: self.data[r][c]
                    });
                }
                // Not a low point, advance to next column
                c = c + 1;
            }
            // Advance to next row
            r = r + 1;
            // And restart scanning from first column
            c = 0;
        }

        return None;
    }
}

fn load_data() -> Board {
    let mut data = Vec::<Vec<u8>>::new();

    if let Ok(lines) = filez::read_lines("inputs/09.txt") {
        for line in lines {
            if let Ok(raw) = line {
                let row = raw.chars()
                    .map(
                        |c| {
                            String::from(c).parse::<u8>().unwrap()
                        }
                    )
                    .collect();
                data.push(row);
            }
        }
    }

    return Board{ 
        r_state: 0,
        c_state: 0,
        data: data,
    };
}

pub fn part1() {
    let board = load_data();

    let mut total_risk_level: u64 = 0;

    for low_point in board {
        total_risk_level += low_point.risk_level() as u64;
    }

    println!("09.1 total risk: {}", total_risk_level);
}

pub fn part2() {
    let board = load_data();
    // Creating a second copy of same data. Could not find a nice way
    // to satisfy the borrow checker inside the closure used by .map
    let board2= load_data();

    let mut basin_sizes: Vec<usize> = board.into_iter()
        .map( |low_point| board2.basin_size(low_point))
        .collect();

    // Sort basin sizes in reverse order
    basin_sizes.sort();
    basin_sizes.reverse();

    let area_to_avoid = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
    println!("09.2 larger basin sizes: {}", area_to_avoid);
}
