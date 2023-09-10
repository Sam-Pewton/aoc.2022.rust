use std::fs;

#[repr(i8)]
enum Outcome {
    WIN = 6,
    LOSS = 0,
    DRAW = 3,
}

#[repr(i8)]
#[derive(Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Game {
    user: Shape,
    opponent: Shape,
}

impl Game {
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

    fn outcome(&self) -> Outcome {
        match self.user as i8 - self.opponent as i8 {
            0 => Outcome::DRAW,
            1 | -2 => Outcome::WIN,
            _ => Outcome::LOSS,
        }
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    println!("Part 1 Score: {}", calculate_score1(&data));
    println!("Part 2 Score: {}", calculate_score2(&data));
}

fn calculate_score1(data: &str) -> u64 {
    data.lines().map(|x| { let y = Game::new1(&x); y.outcome() as u64 + y.user as u64 }).sum()
}

fn calculate_score2(data: &str) -> u64 {
    data.lines().map(|x| { let y = Game::new2(&x); y.outcome() as u64 + y.user as u64 }).sum()
}
