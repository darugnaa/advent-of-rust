use super::filez;

pub fn part1() {
    let input = load_data_3("inputs/03.txt");

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for n in 0..12 {
        let bypos = least_most_common_in_pos(&input, n);
        gamma.push_str(&bypos.0);
        epsilon.push_str(&bypos.1);
    }
    // println!("gamma {}, epsilon {}", gamma, epsilon);
    let gamma_val = isize::from_str_radix(&gamma, 2).unwrap() as u64;
    let epsilon_val = isize::from_str_radix(&epsilon, 2).unwrap() as u64;
    println!("03.1 result: {}", gamma_val * epsilon_val);
}

pub fn part2() {
    let mut input_oxygen = load_data_3("inputs/03.txt");
    let mut input_scrub = input_oxygen.clone();
    for n in 0..12 {
        // println!("--- LOOP {} ---", n);
        if input_oxygen.len() > 1 {
           let oxpos = least_most_common_in_pos(&input_oxygen, n);
           input_oxygen.retain(|x| oxpos.1.eq(&x[n..n+1]));
        }
        // println!("Oxygen measurements: {}", input_oxygen.len());

        if input_scrub.len() > 1 {
            let scrubpos = least_most_common_in_pos(&input_scrub, n);
            input_scrub.retain(|x| scrubpos.0.eq(&x[n..n+1]));
        }
        // println!("ScrubCO2 measurements: {}", input_scrub.len());
    }
    // println!("Oxygen {:?}, Scrubber {:?}", input_oxygen, input_scrub);
    let oxygen = isize::from_str_radix(&input_oxygen[0], 2).unwrap() as u64;
    let scrub = isize::from_str_radix(&input_scrub[0], 2).unwrap() as u64;
    println!("03.2 result: {}", oxygen * scrub);
}

fn least_most_common_in_pos(input: &Vec<String>, pos: usize) -> (String,String) {
    let mut zero_count = 0;
    let mut ones_count = 0;
    for elem in input {
        let char_at = elem.chars().nth(pos).unwrap();
        if char_at == '0' {
            zero_count += 1;
        }
        if char_at == '1' {
            ones_count += 1;
        }
    }

    /*
    if zero_count == ones_count {
        panic!("Same count of 0 and 1");
    }
    * ok to return 1 as most common if numbers are equal
    */

    if zero_count > ones_count {
        return (String::from("1"), String::from("0"))
    } else {
        return (String::from("0"), String::from("1"))
    }
}

fn load_data_3(file_name: &str) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    if let Ok(lines) = filez::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                if num == "" {
                    continue;
                }

                input.push(String::from(num.trim()));
            }
        }
    }
    return input;
}
