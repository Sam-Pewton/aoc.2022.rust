use std::cmp::{Ordering, PartialEq};
use std::collections::VecDeque;
///
/// AOC 2022 Day 12 -> WIP
///
/// This currently only finds all of the connected 'a' values
///
use std::fs;

#[derive(Debug)]
struct Hill {
    map: Vec<Vec<char>>,
    _tracker: VecDeque<Level>,
    current: Level,
}

impl Hill {
    fn new(data: Vec<Vec<char>>, start_coord: Coordinate) -> Self {
        let current = Level::new(&data, start_coord, 0);
        Hill {
            map: data,
            _tracker: VecDeque::new(),
            current,
        }
    }

    fn shortest_climb(&self) {
        // find all of the connected values on the same level
        let mut checked: Vec<Level> = vec![];
        let mut to_check: VecDeque<Level> = VecDeque::new();
        to_check.push_back(self.current.clone());

        while !to_check.is_empty() {
            self.check_neighbours(&mut checked, &mut to_check);
        }
        println!("{:?}", checked);
    }

    fn check_coords(&self, new: Level, checked: &mut Vec<Level>, to_check: &mut VecDeque<Level>) {
        if checked.iter().any(|&x| x.coord == new.coord) {
            let ind = checked.iter().position(|x| x.coord == new.coord).unwrap();
            if checked[ind].steps.cmp(&new.steps) == Ordering::Less {
                checked[ind].steps = new.steps;
            }
        } else if new.level == self.current.level {
            to_check.push_back(new);
        }
    }

    fn check_neighbours(&self, checked: &mut Vec<Level>, to_check: &mut VecDeque<Level>) {
        let current = to_check.pop_front().unwrap();

        // check left and right
        match current.coord.x.cmp(&0) {
            Ordering::Equal => {
                let coord = Coordinate::new(current.coord.x + 1, current.coord.y);
                let new = Level::new(&self.map, coord, current.steps + 1);
                self.check_coords(new, checked, to_check);
            }
            Ordering::Greater => match current.coord.x.cmp(&(self.map[0].len() - 1)) {
                Ordering::Less => {
                    let coord = Coordinate::new(current.coord.x + 1, current.coord.y);
                    let new = Level::new(&self.map, coord, current.steps + 1);
                    self.check_coords(new, checked, to_check);

                    let coord = Coordinate::new(current.coord.x - 1, current.coord.y);
                    let new = Level::new(&self.map, coord, current.steps + 1);
                    self.check_coords(new, checked, to_check);
                }
                _ => (),
            },
            _ => panic!("Index cannot be negative, incorrect input"),
        }

        // check up and down
        match current.coord.y.cmp(&0) {
            Ordering::Equal => {
                let coord = Coordinate::new(current.coord.x, current.coord.y + 1);
                let new = Level::new(&self.map, coord, current.steps + 1);
                self.check_coords(new, checked, to_check);
            }
            Ordering::Greater => match current.coord.y.cmp(&(self.map.len() - 1)) {
                Ordering::Less => {
                    let coord = Coordinate::new(current.coord.x, current.coord.y + 1);
                    let new = Level::new(&self.map, coord, current.steps + 1);
                    self.check_coords(new, checked, to_check);

                    let coord = Coordinate::new(current.coord.x, current.coord.y - 1);
                    let new = Level::new(&self.map, coord, current.steps + 1);
                    self.check_coords(new, checked, to_check);
                }
                _ => (),
            },
            _ => panic!("Index cannot be negative, incorrect input"),
        }
        checked.push(current);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Level {
    level: char,
    coord: Coordinate,
    steps: u8,
}

impl Level {
    fn new(data: &Vec<Vec<char>>, coord: Coordinate, steps: u8) -> Self {
        // 'S' == 'a'
        let level = data[coord.y][coord.x];
        Level {
            level,
            coord,
            steps,
        }
    }
}

fn read_data(filename: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(filename).unwrap();
    let mut data_vec = vec![];
    for line in data.lines() {
        data_vec.push(line.chars().collect::<Vec<char>>());
    }
    data_vec
}

fn find_start_and_end(data: &mut Vec<Vec<char>>) -> Option<(Coordinate, Coordinate)> {
    let start_row = data.iter().map(|x| x.contains(&'S')).position(|x| x)?;
    let start_col = data[start_row].iter().map(|x| x == &'S').position(|x| x)?;
    data[start_row][start_col] = 'a';
    let end_row = data.iter().map(|x| x.contains(&'E')).position(|x| x)?;
    let end_col = data[end_row].iter().map(|x| x == &'E').position(|x| x)?;
    data[end_row][end_col] = 'z';
    Some((
        Coordinate::new(start_col, start_row),
        Coordinate::new(end_col, end_row),
    ))
}

fn main() {
    let mut data = read_data("temp.txt");
    //let mut data = read_data("data.txt");
    let start = find_start_and_end(&mut data).unwrap();
    let hill = Hill::new(data, start.0);
    hill.shortest_climb();
}
