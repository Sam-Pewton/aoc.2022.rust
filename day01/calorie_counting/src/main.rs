// 
use std::fs;

struct Elf {
    id: usize,
    calories: u64,
}

fn main() {
    let file = fs::read_to_string("data.txt");
    let mut elves = generate_elves(&file.unwrap());
    let most = find_most_calories(&mut elves);
    println!("Most calories: {}", most);
    let total = find_top_three_calories(&mut elves, 3);
    println!("Top 3 sum: {}", total);
}

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

fn find_most_calories(elves: &Vec<Elf>) -> u64 {
    elves.iter().fold(0, |x, y| x.max(y.calories))
}

fn find_top_three_calories(elves: &mut Vec<Elf>, top_count: usize) -> u64 {
    elves.sort_by(|a, b| a.calories.partial_cmp(&b.calories).unwrap());
    elves.split_at(elves.iter().count() - top_count).1.iter().map(|x| x.calories).sum()
}
