/// 
/// AOC 2022 Day 2
///
use std::fs;

/// 
/// Outcome enum representing the outcome state of a game.
///
/// The corresponding points associated with the outcome are set behind the state.
///
#[repr(i8)]
enum Outcome {
    WIN = 6,
    LOSS = 0,
    DRAW = 3,
}

/// 
/// Shape enum representing the hand shapes used in the game.
///
/// The corresponding points associated with the shape are set behind the state.
///
#[repr(i8)]
#[derive(Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

/// 
/// Game struct holding the game state. The game state is the shape that each of the players has
/// chosen.
///
struct Game {
    user: Shape,
    opponent: Shape,
}

impl Game {
    /// 
    /// The first part of the day task. Used for calculating the total score when X, Y, and Z
    /// correlate to Shapes.
    ///
    fn new1(data: &str) -> Self {
        let mut split_data = data.split(' ');
        let lhs = match split_data.next() {
            Some("A") => Shape::Rock,
            Some("B") => Shape::Paper,
            Some("C") => Shape::Scissors,
            _ => panic!("Unknown input"),
        };

        let rhs = match split_data.next() {
            Some("X") => Shape::Rock,
            Some("Y") => Shape::Paper,
            Some("Z") => Shape::Scissors,
            _ => panic!("Unknown input"),
        };

        Game { user: rhs, opponent: lhs }
    }

    /// 
    /// The second part of the day task. Used for calculating the total score when X, Y, and Z
    /// correlate to the Outcome required for a game.
    ///
    fn new2(data: &str) -> Self {
        let mut split_data = data.split(' ');

        let lhs = match split_data.next() {
            Some("A") => Shape::Rock,
            Some("B") => Shape::Paper,
            Some("C") => Shape::Scissors,
            _ => panic!("Unknown input"),
        };

        let rhs = match split_data.next() {
            Some("X") => Outcome::LOSS,
            Some("Y") => Outcome::DRAW,
            Some("Z") => Outcome::WIN,
            _ => panic!("Unknown input"),
        };

        let user_input = match rhs {
            Outcome::DRAW => lhs.clone(),
            Outcome::WIN => {
                match lhs {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            },
            Outcome::LOSS => {
                match lhs {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper
                }
            },
        };

        Game { user: user_input, opponent: lhs }
    }

    /// 
    /// Calculate the number of points to award the outcome of a game. The outcome is determined by
    /// comparing the point differences in each of the players shape.
    ///
    /// When the difference is 0, the outcome is a draw.
    /// When the difference is 1 or -2, the outcome is a win for the user.
    /// When the difference is anything else, the outcome is a loss.
    fn outcome(&self) -> Outcome {
        match self.user as i8 - self.opponent as i8 {
            0 => Outcome::DRAW,
            1 | -2 => Outcome::WIN,
            _ => Outcome::LOSS,
        }
    }
}

/// 
/// Entrypoint
///
fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    println!("Part 1 Score: {}", calculate_score1(&data));
    println!("Part 2 Score: {}", calculate_score2(&data));
}

/// 
/// Calculate the overall score across any number of games separated by a new line character in the
/// data.
///
fn calculate_score1(data: &str) -> u64 {
    data.lines().map(|x| { let y = Game::new1(&x); y.outcome() as u64 + y.user as u64 }).sum()
}

/// 
/// Calculate the overall score across any number of games separated by a new line character in the
/// data.
///
fn calculate_score2(data: &str) -> u64 {
    data.lines().map(|x| { let y = Game::new2(&x); y.outcome() as u64 + y.user as u64 }).sum()
}
