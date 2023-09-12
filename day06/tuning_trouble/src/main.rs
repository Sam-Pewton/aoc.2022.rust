///
/// AOC 2022 Day 6
///
use std::fs;
///
/// Custom error for being out of characters
///
#[derive(Debug)]
struct OutOfCharactersError;
///
/// Find a given number of characters in a transmission
///
fn find_marker(
    transmission: &str,
    num_chars: usize,
) -> Result<(Vec<char>, usize), OutOfCharactersError> {
    let mut characters = transmission.chars();
    let mut tracker = num_chars;
    let mut items = (0..num_chars)
        .into_iter()
        .map(|_| characters.next().unwrap())
        .collect::<Vec<char>>();

    for _ in 0..transmission.len() - num_chars {
        let mut items_clone = items.clone();
        items_clone.sort();
        items_clone.dedup();
        if items_clone.len() == num_chars {
            break;
        }
        items.reverse();
        items.pop();
        items.reverse();
        items.push(characters.next().unwrap());
        tracker += 1;
    }
    match tracker == transmission.len() {
        true => Err(OutOfCharactersError),
        false => Ok((items, tracker)),
    }
}
///
/// Entrypoint
///
fn main() {
    let transmission = fs::read_to_string("data.txt").unwrap();
    println!("First packet: {:#?}", find_marker(&transmission, 4));
    println!("First message: {:#?}", find_marker(&transmission, 14));
}
