use super::filez;
use std::collections::HashMap;

/// Returns an HashMap with fish age as key and count of fishes of that age as value.
fn load_data_6() -> HashMap<u8, u64> {
    let mut data = HashMap::<u8, u64>::new();

    if let Ok(raw_data) = filez::read_lines("inputs/06.txt") {
        // Consumes the iterator, returns an Optional String
        for raw_line in raw_data {
            if let Ok(raw) = raw_line {
                let elems: Vec<&str> = raw.split(',').collect();
                for nz in elems {
                    let fish_state: u8 = nz.trim().parse().unwrap();
                    let count = data.entry(fish_state).or_insert(0);
                    let updated_count = *count + 1u64;
                    data.insert(fish_state, updated_count);
                }
            }
        }
    }

    return data;
}

// Executes the fish life for the specified amount of days
fn iterate_fish_life(fish_states: &mut HashMap<u8, u64>, days: u32) {
    for _ in 0..days {
        let mut fish_reset_count = 0u64;
        for timer in 0..=8 {
            if timer == 0 {
                // Fishes of timer 0 will generate a new fish and respawn with timer=6
                let count = fish_states.get(&timer).unwrap_or(&0);
                fish_reset_count = *count;
            }
            // Fetch next group of fish and decrease its timer
            let next_state = timer + 1;
            let count = *fish_states.get(&next_state).unwrap_or(&0);
            fish_states.insert(timer, count);
        }

        // Reset fishes after a duplication cycle by adding the amount of fishes
        // to the ones already at timer 6
        let count = fish_states.get(&6).unwrap_or(&0);
        let aged_six = *count + fish_reset_count;
        fish_states. insert(6, aged_six);
        // Add new fishes with timer 8. Any existing fish with timer 8
        // would be of timer 7 due to previous loop
        fish_states.insert(8, fish_reset_count);
    }
}

/// Returns the total number of fishes in the ocean with timer between 0 and 8 included
/// 
/// # Arguments
/// 
/// * `fish_states` - A HashMap<u8, u64> reference containing number of fishes
fn count_fishes(fish_states: &HashMap<u8, u64>) -> u64 {
    let mut fish_count = 0u64;
    for state in 0..=8 {
        let count = fish_states.get(&state).unwrap_or(&0);
        fish_count = fish_count + *count;
    }
    return fish_count;
}

pub fn part1() {
    let mut fish_states = load_data_6();

    iterate_fish_life(&mut fish_states, 80);
    let fish_count = count_fishes(&fish_states);

    println!("06.1 total fishes: {}", fish_count);
}

pub fn part2() {
    let mut fish_states = load_data_6();

    iterate_fish_life(&mut fish_states, 256);
    let fish_count = count_fishes(&fish_states);

    println!("06.2 total fishes: {}", fish_count);
}
