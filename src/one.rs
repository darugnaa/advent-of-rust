use super::filez;

pub fn part1() {
    let input = load_input("inputs/01.txt");
    // println!("Input has {} lines", input.len());
    let mut current_measure = input[0];
    let mut result: u64 = 0;
    for n in 1..input.len() {
        if input[n] > current_measure {
            result = result + 1;
        }
        current_measure = input[n];
    }
    println!("01.1 increases: {}", result);
}

pub fn part2() {
    let input = load_input("inputs/01.txt");
    // println!("Input has {} lines", input.len());

    let mut current_measure = input[0] + input[1] + input[2];
    let mut result: u64 = 0;
    for n in 1..input.len() - 2 {
        let window = input[n] + input[n+1] + input[n+2];
        if window > current_measure {
            result = result + 1;
        }
        current_measure = window;
    }
    println!("01.2 increases: {}", result);
}

fn load_input(file_name: &str) -> Vec<u64> {
    let mut result1 = Vec::new();
    if let Ok(lines) = filez::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                if num == "" {
                    continue;
                }
                // let guess_n: u32 = guess.trim().parse().expect("Not a number!!!"); // shadowing prev var
                let num: u64 = match num.trim().parse() {
                    // match on io::Result enum
                    Ok(num) => num,
                    Err(x) => {
                        panic!("Unexpected value: {}", x);
                    }
                };
                result1.push(num);
            }
        }
    }

    return result1;
}
