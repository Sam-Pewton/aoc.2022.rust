/// 
/// AOC 2022 Day 4
///
use std::fs;
/// 
/// Section struct housing the beginning and end sections for a given elf
///
struct Sections {
    start: u64,
    end: u64,
}

impl Sections {
    /// 
    /// Create a new Sections instance
    ///
    fn new(data: &str) -> Sections {
        let mut split_data = data.split('-');
        let start = split_data.next().unwrap().parse::<u64>().unwrap();
        let end = split_data.next().unwrap().parse::<u64>().unwrap();
        Sections { start, end }
    }
    /// 
    /// Check that this section fully contains another
    ///
    fn fully_contains(&self, oth: &Sections) -> bool {
        self.start <= oth.start && self.end >= oth.end
    }
    /// 
    /// Check that this section overlaps with another
    ///
    fn overlaps(&self, oth: &Sections) -> bool {
        (self.start >= oth.start && self.start <= oth.end) || 
            (oth.start >= self.start && oth.start <= self.end)
    }
}
/// 
/// Parse a line of data into a tuple of two Sections
///
fn parse_pair(data: &str) -> (Sections, Sections) {
    let mut split_data = data.split(',');
    (Sections::new(split_data.next().unwrap()), Sections::new(split_data.next().unwrap()))
}
/// 
/// Entrypoint
///
fn main() {
    let mut contain_count = 0;
    let mut overlap_count = 0;
    for line in fs::read_to_string("data.txt").unwrap().lines() {
        let pair = parse_pair(line);
        if pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0) {
            contain_count += 1;
        }
        if pair.0.overlaps(&pair.1) {
            overlap_count += 1;
        }
    }
    println!("Total fully contained: {}", contain_count);
    println!("Total overlaps: {}", overlap_count);
}
