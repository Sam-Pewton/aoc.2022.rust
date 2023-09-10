/// 
/// AOC 2022 Day 3
///
/// Not the shortest solution here. This can probably be refactored.
///
use std::fs;

/// 
/// Rucksack struct representing the two compartments in a rucksack
///
struct Rucksack {
    compartment_a: String,
    compartment_b: String,
}

impl Rucksack {
    /// 
    /// Between the two compartments of a rucksack, get the items that appear in both, and
    /// calculate their overall priority.
    ///
    fn get_rucksack_priority(&self) -> u64 {
        self.get_priority_items().iter().map(|x| self.get_item_weight(&x)).sum::<u64>()
    }

    /// 
    /// Get the weight of an character.
    ///
    /// Uses the ascii value of a character using modulo to set the values appropriately.
    ///
    fn get_item_weight(&self, item: &char) -> u64 {
        match item.is_ascii_lowercase() {
            true => *item as u64 % 96,
            false => (*item as u64 % 64) + 26,
        }
    }

    /// 
    /// Get the priority items in a rucksack
    ///
    fn get_priority_items(&self) -> Vec<char> {
        let a_characters: Vec<char> = self.get_unique_items();
        let mut matching: Vec<char> = vec![];
        for character in a_characters {
            if self.compartment_b.contains(character) {
                matching.push(character)
            }
        }
        matching
    }

    /// 
    /// Get a vector of unique items in compartment a
    ///
    fn get_unique_items(&self) -> Vec<char> {
        let mut a_characters: Vec<char> = self.compartment_a.chars().collect();
        a_characters.sort();
        a_characters.dedup();
        a_characters
    }
    
    /// 
    /// Check the rucksack to see if the item exists in either compartment
    ///
    fn find_item(&self, item: &char) -> bool {
        match (self.compartment_a.clone() + &self.compartment_b).contains(*item) {
            true => true,
            false => false,
        }
    }
}

/// 
/// Generate the rucksack data from each line in the data
///
fn generate_rucksacks(data: &str) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = vec![];
    for line in data.lines() {
        if line.len() % 2 != 0 {
            panic!("Incorrect amount of items in rucksack!");
        }
        let split_data = line.split_at(line.len() / 2);
        rucksacks.push( 
            Rucksack { 
                compartment_a: String::from(split_data.0),
                compartment_b: String::from(split_data.1)
            }
        )
    }
    rucksacks
}

/// 
/// Check the rucksack groups for their badge items, and calculate the priority
///
fn check_rucksack_groups(rucksacks: &Vec<Rucksack>, group_size: usize) -> u64 {
    if rucksacks.len() % group_size != 0 {
        panic!("Insufficient group sizes")
    }
    let mut group_split = rucksacks.split_at(group_size);
    let mut weight_total = 0;
    while group_split.1.len() > 0 {
        weight_total += find_badge(group_split.0);
        group_split = group_split.1.split_at(group_size);
    }
    // Add the last iteration as it is missed by the while loop
    weight_total += find_badge(group_split.0);
    weight_total
}

/// 
/// Find the badge item in rucksack groups 
///
fn find_badge(rucksacks: &[Rucksack]) -> u64 {
    for item in (rucksacks[0].compartment_a.clone() + &rucksacks[0].compartment_b).chars() {
        if rucksacks[1].find_item(&item) && rucksacks[2].find_item(&item) {
            return rucksacks[0].get_item_weight(&item);
        };
    }
    panic!("Could not find badge item");
}

/// 
/// Entrypoint
///
fn main() {
    let rucksacks = generate_rucksacks(&fs::read_to_string("data.txt").unwrap());
    println!("Part 1: {}", rucksacks.iter().map(|x| x.get_rucksack_priority()).sum::<u64>());
    println!("Part 2: {}", check_rucksack_groups(&rucksacks, 3));
}
