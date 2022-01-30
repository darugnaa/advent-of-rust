use super::filez;

fn load_data_7() -> Vec<u16> {
    let mut data = Vec::<u16>::new();

    if let Ok(raw_data) = filez::read_lines("inputs/07.txt") {
        // Consumes the iterator, returns an Optional String
        for raw_line in raw_data {
            if let Ok(raw) = raw_line {
                let elems: Vec<&str> = raw.split(',').collect();
                for nz in elems {
                    let crab_position: u16 = nz.trim().parse().unwrap();
                    data.push(crab_position);
                }
            }
        }
    }

    return data;
}

/// Calculates how much fuel is required to move all crabs to the passed position.
///
/// # Arguments
///
/// * `crab_positions` - Pointer to the vector of crab positions, each element is the position of a crab
/// * `pos` - the final position all crabs have to align to
/// * `cost_function` - a function that calculates the fuel usage for the distance travelled
fn calculate_fuel_for_position(crab_positions: &Vec<u16>, pos: usize, cost_function: fn(i32) -> i32) -> u64 {
    if pos >= crab_positions.len() {
        panic!(
            "Calculating fuel usage for position {} outside size of crab army {}",
            pos,
            crab_positions.len(),
        )
    }
    let mut fuel = 0u64;

    for p in 0..crab_positions.len() {
        let positions_moved = crab_positions[p] as i32 - pos as i32;
        let crab_fuel = cost_function(positions_moved.abs());
        fuel += crab_fuel.abs() as u64;
    }

    return fuel;
}

pub fn part1() {
    let crab_positions = load_data_7();

    // Pick insanely high number as starting point
    let mut min_fuel = u64::max_value();

    // Cost function is identity function: number of positions travelled equals fuel used
    let identity_cost_function = |x:i32| x;

    // Brute force through all possible positions
    for i in 0..crab_positions.len() {
        let fuel = calculate_fuel_for_position(&crab_positions, i, identity_cost_function);
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    println!("07.1 min fuel: {}", min_fuel);
}

fn crab_engineering_cost_function(pos: i32) -> i32 {
    let mut result = 0i32;

    for i in 0..=pos {
        result += i;
    }

    return result;
}

pub fn part2() {
    let crab_positions = load_data_7();

    // Pick insanely high number as starting point
    let mut min_fuel = u64::max_value();

    // Brute force through all possible positions
    for i in 0..crab_positions.len() {
        let fuel = calculate_fuel_for_position(&crab_positions, i, crab_engineering_cost_function);
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    println!("07.2 min fuel: {}", min_fuel);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crab_engineering_cost_function() {
        assert_eq!(crab_engineering_cost_function(1), 1);
        assert_eq!(crab_engineering_cost_function(2), 3);
        assert_eq!(crab_engineering_cost_function(3), 6);
        assert_eq!(crab_engineering_cost_function(5), 15);
        assert_eq!(crab_engineering_cost_function(9), 45);
        assert_eq!(crab_engineering_cost_function(11), 66);
    }
}
