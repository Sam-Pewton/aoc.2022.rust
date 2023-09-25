/// 
/// AOC 2022 Day 11
///
/// I don't like how I parsed the input on this one. I need to revisit the parse methods at some
/// point to try and condense it. The LevelTest struct is a little pointless too, and could live in
/// the operation struct - but it is tied up with the parsing.
///
use nom::{bytes, IResult};
use std::fs;

/// 
/// Custom error for when a monkey with a certain ID doesn't exist
///
struct UnknownMonkeyError;

/// 
/// Parse the monkeys ID from the input
///
fn parse_monkey_id(line: &str) -> IResult<&str, u8> {
    let (line, _) = bytes::complete::tag("Monkey ")(line)?;
    let (_, id) = bytes::complete::take_till1(|c| c == ':')(line)?;
    Ok(("", id.parse::<u8>().unwrap()))
}

///
/// Parse a vector of initial worry levels from the input
///
fn parse_items(line: &str) -> IResult<&str, Vec<u64>> {
    let (line, _) = bytes::complete::tag("  Starting items: ")(line)?;
    let items = line
        .split(", ")
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    Ok(("", items))
}

/// 
/// Parse the operation from the input
///
fn parse_operation(line: &str, divisor: u64) -> IResult<&str, Operation> {
    let (line, _) = bytes::complete::tag("  Operation: new = ")(line)?;
    let mut items = line.split(" ");
    let lhs = match items.next().unwrap().parse::<u64>() {
        Ok(x) => Some(x),
        Err(_) => None,
    };
    let op = items.next().unwrap();
    let rhs = match items.next().unwrap().parse::<u64>() {
        Ok(x) => Some(x),
        Err(_) => None,
    };
    Ok(("", Operation::new(lhs, rhs, op, divisor)))
}

/// 
/// Parse the test from the input
///
fn parse_test<'a>(
    testln: &'a str,
    trueln: &'a str,
    falseln: &'a str,
) -> IResult<&'a str, LevelTest> {
    let test_val = bytes::complete::tag("  Test: divisible by ")(testln)?;
    let true_val = bytes::complete::tag("    If true: throw to monkey ")(trueln)?;
    let false_val = bytes::complete::tag("    If false: throw to monkey ")(falseln)?;
    Ok((
        "",
        LevelTest {
            test_value: test_val.0.parse::<u64>().unwrap(),
            true_to: true_val.0.parse::<u8>().unwrap(),
            false_to: false_val.0.parse::<u8>().unwrap(),
        },
    ))
}

/// 
/// Jungle struct, housing all of the monkeys and the product of all modulo values each monkey
/// holds.
///
struct Jungle {
    monkeys: Vec<Monkey>,
    mega_mod: u64,
}

impl Jungle {
    /// 
    /// Create a new jungle instance
    ///
    fn new() -> Self {
        Jungle {
            monkeys: vec![],
            mega_mod: 1,
        }
    }

    /// 
    /// Populate the jungle with new monkeys
    ///
    fn populate(&mut self, data: &str, divisor: u64) {
        let split_data = data.split("\n\n");
        for entry in split_data {
            self.monkeys.push(Monkey::new(entry, divisor));
            self.mega_mod *= self.monkeys.last().unwrap().tester.test_value;
        }
    }

    /// 
    /// Run a round of the simulation
    ///
    fn run_round(&mut self) {
        for m in 0..self.monkeys.len() {
            for _ in 0..self.monkeys[m].items.len() {
                self.monkeys[m].items_inspected += 1;
                let mut item = self.monkeys[m].items.remove(0);
                item = self.monkeys[m].operation.execute(item);
                let index = self.find_monkey_index(self.monkeys[m].tester.run_test(&item));
                if let Ok(x) = index {
                    self.monkeys
                        .get_mut(x)
                        .unwrap()
                        .items
                        .push(item % self.mega_mod);
                } else {
                    panic!("Unknown monkey {}", self.monkeys[m].tester.run_test(&item));
                }
            }
        }
    }

    /// 
    /// Find the index of a monkey that has a certain ID associated
    ///
    fn find_monkey_index(&self, monkey_id: u8) -> Result<usize, UnknownMonkeyError> {
        for (i, monkey) in self.monkeys.iter().enumerate() {
            if monkey.id == monkey_id {
                return Ok(i);
            }
        }
        Err(UnknownMonkeyError)
    }

    /// 
    /// Calculate the level of shenanigans.
    ///
    fn level_of_shenanigans(&mut self) -> u64 {
        self.monkeys
            .sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));
        self.monkeys[0].items_inspected * self.monkeys[1].items_inspected
    }
}

/// 
/// Monkey struct, holding all the data for a single monkey
///
struct Monkey {
    id: u8,
    items: Vec<u64>,
    items_inspected: u64,
    operation: Operation,
    tester: LevelTest,
}

impl Monkey {
    /// 
    /// Create a new monkey from the data item
    ///
    fn new(data: &str, divisor: u64) -> Self {
        let mut data_lines = data.lines();
        let (_, monkey_id) = parse_monkey_id(data_lines.next().unwrap()).unwrap();
        let (_, items) = parse_items(data_lines.next().unwrap()).unwrap();
        let (_, operation) = parse_operation(data_lines.next().unwrap(), divisor).unwrap();
        let l1 = data_lines.next().unwrap();
        let l2 = data_lines.next().unwrap();
        let l3 = data_lines.next().unwrap();
        let (_, tester) = parse_test(l1, l2, l3).unwrap();
        Monkey {
            id: monkey_id,
            items,
            items_inspected: 0,
            operation,
            tester,
        }
    }
}

///
/// LevelTest struct used to find which monkey to pass the item to
///
struct LevelTest {
    test_value: u64,
    true_to: u8,
    false_to: u8,
}

impl LevelTest {
    /// 
    /// Run the test
    ///
    fn run_test(&self, value: &u64) -> u8 {
        if value % self.test_value == 0 {
            return self.true_to;
        }
        self.false_to
    }
}

struct Operation {
    lhs: Option<u64>,
    rhs: Option<u64>,
    symbol: Symbol,
    divisor: u64,
}

impl Operation {
    /// 
    /// Create a new operation, this defines the calculation for the new worry level
    ///
    fn new(lhs: Option<u64>, rhs: Option<u64>, symbol: &str, divisor: u64) -> Self {
        match symbol {
            "+" => Operation {
                lhs,
                rhs,
                symbol: Symbol::ADD,
                divisor,
            },
            "*" => Operation {
                lhs,
                rhs,
                symbol: Symbol::MULTIPLY,
                divisor,
            },
            _ => panic!("Unknown symbol encountered"),
        }
    }

    ///
    /// Execute the calculation
    ///
    fn execute(&self, old: u64) -> u64 {
        match self.symbol {
            Symbol::ADD => (self.lhs.unwrap_or(old) + self.rhs.unwrap_or(old)) / self.divisor,
            Symbol::MULTIPLY => (self.lhs.unwrap_or(old) * self.rhs.unwrap_or(old)) / self.divisor,
        }
    }
}

/// 
/// Symbol enum, for the symbol used in the operation calculation
///
enum Symbol {
    ADD,
    MULTIPLY,
}

/// 
/// Run the simulation for x amount of rounds using a custom worry level divisor
///
fn run_simulation(data: &str, rounds: u64, divisor: u64) {
    let mut jungle = Jungle::new();
    jungle.populate(&data, divisor);
    for _ in 0..rounds {
        jungle.run_round();
    }
    println!("{:#?}", jungle.level_of_shenanigans());
}

///
/// Entrypoint
///
fn main() {
    let data = fs::read_to_string("temp.txt").unwrap();
    run_simulation(&data, 20, 3);
    run_simulation(&data, 10000, 1);
}
