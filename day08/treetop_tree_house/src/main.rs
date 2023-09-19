///
/// AOC 2022 Day 8
///
use std::{fs, thread};
use std::sync::{Arc, Mutex};

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
/// Check scenic scores
///
/// This multithreaded function calculates every scenic score and keeps track of the one which
/// scored the highest through a mutex lock. The highest score is returned once all of the threads
/// have finished execution.
///
/// I would have preferred to have one lined the calculations of up, down, left, and right..
///
fn check_scenic_scores(data: &Vec<Vec<u32>>) -> usize {
    // the data is immutable, the max_trees is mutable
    let safe_data = Arc::new(data.clone());
    let max_trees = Arc::new(Mutex::new(0));

    // A new thread for every row
    let mut handles = vec![];
    for i in 0..data.len() {
        let safe = Arc::clone(&safe_data);
        let max_trees = Arc::clone(&max_trees);
        let handle = thread::spawn(move || {
            let a = &safe[i];
            for j in 0..a.len() {
                let mut left = (0..j)
                    .into_iter()
                    .map(|x| a[x] >= a[j])
                    .rev()
                    .map_while(|x| if !x { Some(x) } else { None })
                    .count();
                left = if left < j { left + 1 } else { left };

                let mut up = (0..i)
                    .into_iter()
                    .map(|x| safe[x][j] >= a[j])
                    .rev()
                    .map_while(|x| if !x { Some(x) } else { None })
                    .count();
                up = if up < i { up + 1 } else { up };

                let mut right = (j + 1..a.len())
                    .into_iter()
                    .map(|x| a[x] >= a[j])
                    .map_while(|x| if !x { Some(x) } else { None })
                    .count();
                right = if right + j + 1 < a.len() {
                    right + 1
                } else {
                    right
                };

                let mut down = (i + 1..safe.len())
                    .into_iter()
                    .map(|x| safe[x][j] >= a[j])
                    .map_while(|x| if !x { Some(x) } else { None })
                    .count();
                down = if down + i + 1 < safe.len() {
                    down + 1
                } else {
                    down
                };

                let trees = left * down * right * up;
                let mut c = max_trees.lock().unwrap();
                if trees > *c {
                    *c = trees;
                }
            }
        });
        handles.push(handle);
    }

    for i in handles {
        i.join().unwrap();
    }
    *max_trees.clone().lock().unwrap()
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
    data_vec = transpose(data_vec);
    println!("Part 2: {}", check_scenic_scores(&data_vec));
}
