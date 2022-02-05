use super::filez;

struct Entry {
    signals: Vec<String>,
    outputs: Vec<String>,
}

const LEN_OF_1: usize = 2;
const LEN_OF_4: usize = 4;
const LEN_OF_7: usize = 3;
const LEN_OF_8: usize = 7;

impl Entry {
    fn count_easy_digits(&self) -> u64 {
        let mut digit_count: usize = 0;
        for output in &self.outputs {
            match output.len() {
                LEN_OF_1 | LEN_OF_4 | LEN_OF_7 | LEN_OF_8 => digit_count += 1,  
                _ => (),
            }
        }
        return digit_count as u64;
    }
}


fn load_data() -> Vec<Entry> {
    let mut entries = Vec::<Entry>::new();

    if let Ok(lines) = filez::read_lines("inputs/08.txt") {
        for line in lines {
            if let Ok(raw) = line {
                let elems: Vec<&str> = raw.split('|').collect();
                let signals = elems[0].trim()
                    .split(" ")
                    .map(|s| String::from(s))  // take a &str and make it into a String
                    .collect();
                let outputs = elems[1].trim()
                    .split(" ")
                    .map(|s| String::from(s))
                    .collect();
                entries.push(
                    Entry { signals, outputs }
                );
            }
        }
    }

    return entries;
}

pub fn part1() {
    let entries = load_data();
    let mut result = 0u64;
    for entry in entries {
        result += entry.count_easy_digits();
    }

    println!("08.1 easy digits count: {}", result);
}

pub fn part2() {
    println!("08.2 --- no idea ---");
}
