use super::filez;
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl Line {

    fn is_horizontal(&self) -> bool {
        return self.y1 == self.y2;
    }

    fn is_vertical(&self) -> bool {
        return self.x1 == self.x2;
    }
}

struct IntersectionCounter {
    countmap: HashMap::<u16, HashMap<u16, u16>>,
}

impl IntersectionCounter {
    fn update(&mut self, l: &Line) {
        if l.is_horizontal() {
            // Horizontal lines have fixed Y coordinate and only X changes
            let y = l.y1;
            // Rust iterators only count "up" and not "down"
            // an iterator on 10...4 does not work. (or maybe it does and I don't know it yet)
            let x_range = if l.x1 < l.x2 {
                l.x1..=l.x2
            } else {
                l.x2..=l.x1
            };

            for x in x_range {
                let ycountmap = self.countmap
                    .entry(x)
                    .or_insert(HashMap::new());
                let c = ycountmap.entry(y).or_insert(0);
                let updated_c = *c + 1;
                ycountmap.insert(y, updated_c);
            }
            return;
        }
        if l.is_vertical() {
            // Vertical lines have fixed X coordinate and only Y changes
            let x = l.x1;
            let y_range = if l.y1 < l.y2 {
                l.y1..=l.y2
            } else {
                l.y2..=l.y1
            };

            for y in y_range {
                let ycountmap = self.countmap
                    .entry(x)
                    .or_insert(HashMap::new());
                let c = ycountmap.entry(y).or_insert(0);
                let updated_c = *c + 1;
                ycountmap.insert(y, updated_c);
            }
            return;
        }
        // Line is 45 degrees, verify it

        let x_direction = if l.x1 < l.x2 {
            1
        } else {
            -1
        };
        let y_direction = if l.y1 < l.y2 {
            1
        } else {
            -1
        };
        // Looping on the difference between begin and end of a segment.
        // When looping on a line like \ going "positive" direction as both X and Y increase
        // When looping on a line like / going "negative" direction as both X and Y decrease
        for z in 0..=(l.x1 as i32 - l.x2 as i32).abs() {
            let x = (l.x1 as i32 + (z as i32 * x_direction)) as u16;
            let y = (l.y1 as i32 + (z as i32 * y_direction)) as u16;
            let ycountmap = self.countmap
                .entry(x)
                .or_insert(HashMap::new());
            let c = ycountmap.entry(y).or_insert(0);
            let updated_c = *c + 1;
            ycountmap.insert(y, updated_c);
        }
    }

    fn total(&self) -> u64 {
        let mut t = 0;

        for (_, ycountmap) in &self.countmap {
            for (_, count) in ycountmap {
                if *count > 1 {
                    t = t + 1;
                }
            }
        }

        return t;
    }
}

fn load_data_5() -> Vec<Line> {
    let mut lines = Vec::<Line>::new();

    if let Ok(raw_data) = filez::read_lines("inputs/05.txt") {
        // Consumes the iterator, returns an Optional String
        for raw_line in raw_data {
            if let Ok(raw) = raw_line {
                // line is in the form x1,y1 -> x2,y2
                let from_to: Vec<&str> = raw.split(" -> ").collect();
                // from is now x1,y1
                let from: Vec<&str> = from_to[0].split(",").collect();
                let x1: u16 = from[0].parse().unwrap();
                let y1: u16 = from[1].parse().unwrap();
                // to is now x2,y2
                let to: Vec<&str> = from_to[1].split(",").collect();
                let x2: u16 = to[0].parse().unwrap();
                let y2: u16 = to[1].parse().unwrap();

                let line = Line {
                    x1: x1, y1: y1, x2: x2, y2: y2,
                };

                lines.push(line);
            }
        }
    }

    return lines;
}


pub fn part1() {
    let lines = load_data_5();
    // Select only horizontal and vertical lines
    let lines = lines.iter()
        .filter(|&l| l.is_horizontal() || l.is_vertical());

    let mut intersection_counter = IntersectionCounter {
        countmap: HashMap::<u16, HashMap<u16, u16>>::new()
    };

    for l in lines {
        intersection_counter.update(l);
    }

    println!("05.1 intersections: {}", intersection_counter.total());
}


pub fn part2() {
    let lines = load_data_5();
    
    let mut intersection_counter = IntersectionCounter {
        countmap: HashMap::<u16, HashMap<u16, u16>>::new()
    };

    for l in lines {
        intersection_counter.update(&l);
    }

    println!("05.2 intersections: {}", intersection_counter.total());
}