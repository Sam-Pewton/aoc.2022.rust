/// 
/// AOC 2022 Day 5
///
use std::{fs,collections::HashMap};

/// 
/// Command struct, stores the command requirements for moving x stacks from a to b
///
struct Command {
    quantity: u32,
    from: char,
    to: char,
}
/// 
/// Stacks struct, houses the state of all of the stacks
///
struct Stacks {
    stacks: HashMap<char, Vec<char>>,
}

impl Stacks {
    /// 
    /// Create a new Stacks instance
    ///
    fn new(data: &str) -> Stacks {
        let mut stacks: HashMap<char, Vec<char>> = HashMap::new();
        let mut data_iterator = data.lines().rev();

        let keys = data_iterator.next().unwrap();
        let indices: Vec<usize> = keys.chars().enumerate()
            .filter(|(_, x)| *x != ' ')
            .map(|(i, _)| i)
            .collect();

        for ind in indices.iter() {
            let vec_vals: Vec<char> = data_iterator.clone()
                .map(|x| x.chars().nth(*ind).unwrap())
                .filter(|x| *x != ' ')
                .collect();
            stacks.insert(keys.chars().nth(*ind).unwrap(), vec_vals);
        }

        Stacks { stacks }
    }
    /// 
    /// Move crates using the cratemover 9000
    ///
    fn cratemover_9000(&mut self, cmd: &Command) {
        for _ in (0..cmd.quantity).collect::<Vec<u32>>() {
            let val = self.stacks.get_mut(&cmd.from).unwrap().pop().unwrap();
            self.stacks.get_mut(&cmd.to).unwrap().push(val);
        }
    }
    /// 
    /// Move crates using the cratemover 9001
    ///
    fn cratemover_9001(&mut self, cmd: &Command) {
        let len = self.stacks.get(&cmd.from).unwrap().len() as u32;
        let mut vals = self.stacks.get_mut(&cmd.from).unwrap().split_off((len - cmd.quantity) as usize);

        self.stacks.get_mut(&cmd.to).unwrap().append(&mut vals);
    }
    /// 
    /// Print the top crate label in each of the stacks
    ///
    fn print_stack_heads(&self) {
        let mut keys = self.stacks.keys().map(|x| x.clone()).collect::<Vec<char>>();
        keys.sort();
        print!("Stack Heads: ");
        for key in keys {
            print!("[{:#?}]", self.stacks.get(&key).unwrap().last().unwrap());
        }
        print!("\n");
    }
}

impl Command {
    /// 
    /// Parse a new command from a line of the input data
    ///
    fn new(data: &str) -> Command {
        let split_data: Vec<&str> = data.split(' ').into_iter().enumerate()
            .filter(|(i, _)| i % 2 != 0)
            .map(|(_, x)| x)
            .collect();
        Command { 
            quantity: split_data[0].parse::<u32>().unwrap(),
            from: split_data[1].parse::<char>().unwrap(),
            to: split_data[2].parse::<char>().unwrap()
        }
    }
}
/// 
/// Part 1 of the exercise
///
fn part1() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut data_split = data.split("\n\n");
    let mut stacks = Stacks::new(data_split.next().unwrap());

    for cmd in data_split.next().unwrap().lines() {
        stacks.cratemover_9000(&Command::new(&cmd));
    }
    stacks.print_stack_heads();
}
/// 
/// Part 2 of the exercise
///
fn part2() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut data_split = data.split("\n\n");
    let mut stacks = Stacks::new(data_split.next().unwrap());

    for cmd in data_split.next().unwrap().lines() {
        stacks.cratemover_9001(&Command::new(&cmd));
    }
    stacks.print_stack_heads();
}
/// 
/// Entrypoint
///
fn main() {
    part1();
    part2();
}
