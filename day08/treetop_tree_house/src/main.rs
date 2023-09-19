/// 
/// AOC 2022 Day 8 - WIP -> Task 1 finished
///
use std::fs;

///
/// Transpose a 2D vector
///
fn transpose<T>(data: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    (0..data[0].len())
        .map(|col| (0..data.len()).map(|row| data[row][col]).collect())
        .collect()
}

///
/// Transform the data read from a file into a 2D vector of u32s.
///
fn transform(data: String) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|char| char as u32 - '0' as u32)
                .collect::<Vec<u32>>()
        })
        .collect()
}

///
/// Check each row and mark off how many numbers are higher than the last found, in the tracker.
///
fn check_row(data_row: &mut Vec<u32>, tracker_row: &mut Vec<bool>) {
    for _ in 0..2 {
        let mut current_top = data_row[0];
        tracker_row[0] = true;
        for c in 1..data_row.len() {
            if data_row[c] > current_top {
                tracker_row[c] = true;
                current_top = data_row[c];
            }
        }
        data_row.reverse();
        tracker_row.reverse();
    }
}

///
/// Entrypoint
///
fn main() {
    let mut data_vec = transform(fs::read_to_string("data.txt").unwrap());
    let mut tracker = vec![vec![false; data_vec[0].len()]; data_vec.len()];
    for i in 0..data_vec.len() {
        check_row(&mut data_vec[i], &mut tracker[i]);
    }
    data_vec = transpose(data_vec);
    tracker = transpose(tracker);
    for i in 0..data_vec.len() {
        check_row(&mut data_vec[i], &mut tracker[i]);
    }
    println!(
        "Part 1: {}",
        tracker
            .iter()
            .map(|row| row.iter().map(|x| *x as u32).sum::<u32>())
            .sum::<u32>()
    );
}
