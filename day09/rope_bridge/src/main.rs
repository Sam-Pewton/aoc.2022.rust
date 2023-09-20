/// 
/// AOC 2022 Day 9 -> WIP
///
/// Task 1 completed, for task 2 I need to add a new MIDDLE kind in the RopeEndKind,
/// add a vector of middle sections to the Rope, and iteratively update each of the items in the
/// rope based on the previous
///
/// Perhaps in the constructor I can add a parameter to create the number of middle sections, then
/// instead of the head and tail vars, I can just have a vec of knots:
/// vec![ HEAD, MIDDLE, MIDDLE, TAIL ], then the tail can be read by getting .last()
///
/// Could probably make this faster by adding a flag to RopeEnd that specifies if we want to track
/// the visited or not too, so that we can only track the end.
use std::fs;
use std::ops::Sub;
use std::cmp::Ordering;
use nom::{
    IResult,
    character,
    bytes::complete::tag
};

fn parse_command(i: &str) -> IResult<&str, i64> {
    let dir = character::complete::alpha0(i)?;
    let space = tag(" ")(dir.0)?;
    let dist = character::complete::i64(space.0)?;
    Ok((dir.1, dist.1))
}

#[derive(Debug, Clone, Copy)]
enum RopeEndKind {
    HEAD,
    TAIL,
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone)]
struct RopeEnd {
    _edge_type: RopeEndKind,
    position: Coordinate,
    visited: Vec<Coordinate>,
}

impl RopeEnd {
    fn new(edge_type: RopeEndKind) -> Self {
        RopeEnd {
            _edge_type: edge_type,
            position: Coordinate { x: 0, y: 0 },
            visited: vec![Coordinate { x: 0, y: 0 }],
        }
    }

    fn move_one(&mut self, direction: &str) {
        match direction {
            "U" => self.position.y += 1,
            "D" => self.position.y -= 1,
            "R" => self.position.x += 1,
            "L" => self.position.x -= 1,
            "UL" => { self.position.y += 1; self.position.x -= 1; }
            "DL" => { self.position.y -= 1; self.position.x -= 1; }
            "UR" => { self.position.y += 1; self.position.x += 1; } 
            "DR" => { self.position.y -= 1; self.position.x += 1; }
            _ => (),
        };
        if !self.visited.contains(&self.position) {
            self.visited.push(self.position.clone());
        }
    }
}

impl Sub for RopeEnd {
    type Output = i64;
    fn sub(self, other: Self) -> Self::Output {
        let x_diff = (other.position.x - self.position.x).abs();
        let y_diff = (other.position.y - self.position.y).abs();
        if x_diff == 1 && y_diff == 1 {
            return 1;
        };
        x_diff + y_diff
    }
}

#[derive(Debug)]
struct Rope {
    head: RopeEnd,
    tail: RopeEnd,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: RopeEnd::new(RopeEndKind::HEAD),
            tail: RopeEnd::new(RopeEndKind::TAIL),
        }
    }
    
    fn move_direction(&mut self, direction: &str, amount: i64) {
        for _ in 0..amount {
            self.head.move_one(direction);
            match self.head.clone().sub(self.tail.clone()) {
                0 | 1 => (),
                _ => {
                    self.tail_mover();
                }
            }
        }
    }

    fn tail_mover(&mut self) {
        let head = &self.head.position;
        let tail = &self.tail.position;
        match (head.x.cmp(&tail.x), head.y.cmp(&tail.y)) {
            (Ordering::Equal, Ordering::Less) => self.tail.move_one("D"),
            (Ordering::Equal, Ordering::Greater) => self.tail.move_one("U"),
            (Ordering::Less, Ordering::Equal) => self.tail.move_one("L"),
            (Ordering::Greater, Ordering::Equal) => self.tail.move_one("R"),
            (Ordering::Less, Ordering::Less) => self.tail.move_one("DL"),
            (Ordering::Greater, Ordering::Greater) => self.tail.move_one("UR"),
            (Ordering::Greater, Ordering::Less) => self.tail.move_one("DR"),
            (Ordering::Less, Ordering::Greater) => self.tail.move_one("UL"),
            _ => (),
        }
    }
}

fn main() {
    let mut rope = Rope::new();
    let data = fs::read_to_string("data.txt").unwrap();
    for line in data.lines() {
        let cmd = parse_command(&line).unwrap();
        rope.move_direction(cmd.0, cmd.1)
    }
    println!("Part 1: {}", rope.tail.visited.len());
}
