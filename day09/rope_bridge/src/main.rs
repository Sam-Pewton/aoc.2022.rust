/// 
/// AOC 2022 Day 9
///
use std::fs;
use std::ops::Sub;
use std::cmp::Ordering;
use nom::{
    IResult,
    character,
    bytes::complete::tag
};

/// 
/// KnotType, defining where on the rope the knot is.
///
#[derive(Debug, Clone, Copy)]
enum KnotType {
    HEAD,
    MIDDLE,
    TAIL,
}

/// 
/// Coordinate, to hold the coordinate of the knot
///
#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl PartialEq for Coordinate {
    /// 
    /// Compare one coordinate to another for equivalence
    ///
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// 
/// Knot struct, holding data about each knot
///
#[derive(Debug, Clone)]
struct Knot {
    _knot_type: KnotType,
    position: Coordinate,
    track_visited: bool,
    visited: Vec<Coordinate>,
}

impl Knot {
    /// 
    /// Create a new knot instance
    ///
    fn new(edge_type: KnotType, track_visited: bool) -> Self {
        Knot {
            _knot_type: edge_type,
            position: Coordinate { x: 0, y: 0 },
            track_visited,
            visited: vec![Coordinate { x: 0, y: 0 }],
        }
    }

    /// 
    /// Move the knot a distance of 1 in a given direction
    ///
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
        // If we are tracking the knot and it is a new coord, add it
        if !self.visited.contains(&self.position) && self.track_visited {
            self.visited.push(self.position.clone());
        }
    }
}

impl Sub for Knot {
    type Output = i64;
    /// 
    /// Subtract one knot from the other, determining the difference between the two knots.
    ///
    /// Takes the absolute of the difference. If the difference is more than 1, the tail needs to
    /// move (or if both differences are 1, this is a diagonal)
    ///
    fn sub(self, other: Self) -> Self::Output {
        let x_diff = (other.position.x - self.position.x).abs();
        let y_diff = (other.position.y - self.position.y).abs();
        if x_diff == 1 && y_diff == 1 {
            return 1;
        };
        x_diff + y_diff
    }
}

/// 
/// Rope struct, holding all of the knots
///
#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    /// 
    /// Create a new rope, with x amount of middle knots.
    ///
    fn new(middle_knots: u8) -> Self {
        let mut knots = vec![Knot::new(KnotType::HEAD, false)];
        for _ in 0..middle_knots {
            knots.push(Knot::new(KnotType::MIDDLE, false));
        }
        knots.push(Knot::new(KnotType::TAIL, true));
        Rope { knots }
    }
    
    /// 
    /// Move the head knot of the rope, and move the trailing knots as required inline with the
    /// movement rules.
    ///
    fn move_direction(&mut self, direction: &str, amount: i64) {
        for _ in 0..amount {
            self.knots[0].move_one(direction);
            for i in 1..self.knots.len() {
                match self.knots[i - 1].clone().sub(self.knots[i].clone()) {
                    0 | 1 => (),
                    _ => self.knot_mover(i - 1, i),
                }
            }
            
        }
    }

    /// 
    /// Move the trailing knot based on the leading knots position
    ///
    fn knot_mover(&mut self, leading: usize, trailing: usize) {
        let lead = &self.knots[leading].position;
        let trail = &self.knots[trailing].position;
        match (lead.x.cmp(&trail.x), lead.y.cmp(&trail.y)) {
            (Ordering::Equal, Ordering::Less) => self.knots[trailing].move_one("D"),
            (Ordering::Equal, Ordering::Greater) => self.knots[trailing].move_one("U"),
            (Ordering::Less, Ordering::Equal) => self.knots[trailing].move_one("L"),
            (Ordering::Greater, Ordering::Equal) => self.knots[trailing].move_one("R"),
            (Ordering::Less, Ordering::Less) => self.knots[trailing].move_one("DL"),
            (Ordering::Greater, Ordering::Greater) => self.knots[trailing].move_one("UR"),
            (Ordering::Greater, Ordering::Less) => self.knots[trailing].move_one("DR"),
            (Ordering::Less, Ordering::Greater) => self.knots[trailing].move_one("UL"),
            _ => (),
        }
    }
}

/// 
/// Parse the command a line of the input
///
fn parse_command(i: &str) -> IResult<&str, i64> {
    let dir = character::complete::alpha0(i)?;
    let space = tag(" ")(dir.0)?;
    let dist = character::complete::i64(space.0)?;
    Ok((dir.1, dist.1))
}

/// 
/// Run all of the commands on a new rope with x middle knots
///
fn run_commands(middle_knots: u8, data: &str) -> Rope {
    let mut rope = Rope::new(middle_knots);
    for line in data.lines() {
        let cmd = parse_command(&line).unwrap();
        rope.move_direction(cmd.0, cmd.1)
    }
    rope
}

/// 
/// Entrypoint
///
fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let rope = run_commands(0, &data);
    println!("Part 1: {}", rope.knots.last().unwrap().visited.len());
    let rope = run_commands(8, &data);
    println!("Part 2: {}", rope.knots.last().unwrap().visited.len());
}
