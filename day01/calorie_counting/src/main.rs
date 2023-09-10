/// 
/// AOC 2022 Day 1
///
use std::fs;

/// 
/// Elf struct representing the total calories carried by an individual elf.
///
struct Elf {
    id: usize,
    calories: u64,
}

/// 
/// Entrypoint
/// 
fn main() {
    let file = fs::read_to_string("data.txt");
    let mut elves = generate_elves(&file.unwrap());
    let most = find_most_calories(&mut elves);
    println!("Most calories: {}", most);
    let total = find_top_n_calories(&mut elves, 3);
    println!("Top 3 sum: {}", total);
}

/// 
/// Generate a vector of elves using the text data supplied.
///
/// Each elfs inventory is separated by 2 newline characters in succession, and each item in the
/// inventory separated by 1.
/// Each elf inventory is iterated over, parsing each item to an integer, and summing the total.
/// This data is then used to genetare a new elf and is then pushed to the elves vector.
///
fn generate_elves(data: &str) -> Vec<Elf> {
    let split_data = data.split("\n\n");
    let mut elves: Vec<Elf> = vec![];

    for (i, elf_data) in split_data.into_iter().enumerate() {
        let itemized = elf_data.split("\n");
        let total: u64 = itemized
            .map(|x| {x.parse::<u64>()})
            .filter(|x| x.is_ok())
            .map(|x| {x.unwrap()})
            .sum();
        elves.push( Elf { id: i, calories: total } )
        
    }
    elves
}

/// 
/// Find the elf carrying the most amount of calories.
///
fn find_most_calories(elves: &Vec<Elf>) -> u64 {
    elves.iter().fold(0, |x, y| x.max(y.calories))
}

/// 
/// Find the cumulative sum of calories carried by the top n number of elves.
///
/// The elves are first sorted by their individual calorie totals, and the elves are split by the
/// amount that we need to sum. From here, the calories are then summed for the top n elves.
///
fn find_top_n_calories(elves: &mut Vec<Elf>, top_count: usize) -> u64 {
    elves.sort_by(|a, b| a.calories.partial_cmp(&b.calories).unwrap());
    elves.split_at(elves.iter().count() - top_count).1.iter().map(|x| x.calories).sum()
}
