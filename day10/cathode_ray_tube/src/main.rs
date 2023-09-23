///
/// AOC Day 10
///
use nom::{bytes, character, IResult};
use std::fs;

/// 
/// Parse an instruction from a given command.
///
fn parse_instruction(cmd: &str) -> IResult<&str, &str> {
    character::complete::alpha0(cmd)
}

/// 
/// Parse a number from a given command.
///
fn parse_number(cmd: &str) -> IResult<&str, i32> {
    let a = bytes::complete::tag(" ")(cmd)?;
    let b = character::complete::i32(a.0)?;
    Ok((a.1, b.1))
}

/// 
/// Cpu struct to control the execution of commands and update internal memory
///
struct Cpu {
    cycle: i32,
    inspect_elements: Vec<i32>,
    memory: Memory,
}

impl Cpu {
    /// 
    /// Create a new default CPU instance
    ///
    fn new(inspect_elements: Vec<i32>) -> Self {
        Cpu {
            cycle: 0,
            inspect_elements,
            memory: Memory::new(),
        }
    }

    /// 
    /// Execute an instruction on the CPU - instructions are pre-defined in the Instruction enum
    ///
    /// An unknown instruction will cause the kernel to panic!
    ///
    fn execute_instruction(&mut self, instruction: &str) {
        let command = parse_instruction(instruction).unwrap();
        let command = match command.1 {
            "addx" => Instruction::ADDX {
                cycles: 2,
                value: parse_number(command.0).unwrap().1,
            },
            "noop" => Instruction::NOOP { cycles: 1 },
            _ => panic!("Unknown instruction"),
        };

        match command {
            Instruction::ADDX { cycles, value } => {
                for i in 0..cycles {
                    self.cycle += 1;
                    if self.memory.check_sprite((self.cycle - 1) % 40) {
                        self.memory
                            .update_crt((self.cycle - 1) / 40, (self.cycle - 1) % 40);
                    }
                    if self.inspect_elements.contains(&self.cycle) {
                        self.memory.tracked_sum += self.memory.register * self.cycle;
                    }
                    if i == cycles - 1 {
                        self.memory.addx(value);
                    }
                }
            }
            Instruction::NOOP { cycles } => {
                self.cycle += cycles;
                if self.memory.check_sprite((self.cycle - 1) % 40) {
                    self.memory
                        .update_crt((self.cycle - 1) / 40, (self.cycle - 1) % 40);
                }
                if self.inspect_elements.contains(&self.cycle) {
                    self.memory.tracked_sum += self.memory.register * self.cycle;
                }
            }
        }
    }

    /// 
    /// Render the current state of the crt
    ///
    fn render_crt(&self) {
        for row in &self.memory.crt {
            println!("{:?}", row.into_iter().collect::<String>());
        }
    }
}

/// 
/// Instruction enum, holding the structs to map possible instructions
///
enum Instruction {
    ADDX { cycles: i32, value: i32 },
    NOOP { cycles: i32 },
}

/// 
/// Memory struct to track the internal memory state of the communication system
///
struct Memory {
    register: i32,
    tracked_sum: i32,
    crt: Vec<Vec<char>>,
}

impl Memory {
    /// 
    /// Create a new default memory instance
    ///
    fn new() -> Self {
        Memory {
            register: 1,
            tracked_sum: 0,
            crt: vec![vec!['.'; 40]; 6],
        }
    }

    /// 
    /// Add the value to the register
    ///
    fn addx(&mut self, value: i32) {
        self.register += value;
    }

    /// 
    /// Check that the sprite sits on the pixel in render
    ///
    fn check_sprite(&self, pixel_in_render: i32) -> bool {
        (self.register - 1..=self.register + 1).contains(&pixel_in_render)
    }

    /// 
    /// Update the crt pixel to #
    ///
    fn update_crt(&mut self, row: i32, col: i32) {
        self.crt[row as usize][col as usize] = '#';
    }
}

fn main() {
    let mut cpu = Cpu::new(vec![20, 60, 100, 140, 180, 220]);
    let data = fs::read_to_string("data.txt").unwrap();
    for line in data.lines() {
        cpu.execute_instruction(line);
    }
    println!("Part 1: {}", cpu.memory.tracked_sum);
    println!("Part 2:");
    cpu.render_crt();
}
