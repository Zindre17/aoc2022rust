use std::{collections::VecDeque, fmt::Display};

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec11");
    aoc_helper::print_solution(1, part1, INPUT);
}

fn part1(input: &str) -> Option<i32> {
    let mut monkeys = Vec::<Monkey>::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Monkey") {
            let items = lines
                .next()
                .unwrap()
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split(",");

            let operation_text = lines
                .next()
                .unwrap()
                .split("= old")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .collect::<Vec<&str>>();
            let operation = match operation_text[0] {
                "+" => Operation::Add(operation_text[1].parse::<i32>().unwrap()),
                _ if operation_text[1] == "old" => Operation::Power,
                _ => Operation::Multiply(operation_text[1].parse::<i32>().unwrap()),
            };

            let test = lines
                .next()
                .unwrap()
                .split("by")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap();

            let true_target = lines
                .next()
                .unwrap()
                .split("monkey")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap() as usize;
            let false_target = lines
                .next()
                .unwrap()
                .split("monkey")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap() as usize;
            let mut monkey = Monkey::new(operation, test, true_target, false_target);

            for item in items {
                monkey.items.push_back(item.trim().parse::<i32>().unwrap());
            }
            monkeys.push(monkey);
        }
    }

    // for i in 0..monkeys.len() {
    //     println!("{}{}", i, monkeys[i]);
    // }

    // let the games begin
    let mut inspections: Vec<i32> = vec![0; monkeys.len()];
    for _ in 0..20 {
        let mut throws = VecDeque::<Throw>::new();

        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            while monkey.items.len() > 0 {
                let throw = monkey.throw_next_item();
                throws.push_back(throw);
                inspections[i] += 1;
            }
            while let Some((target, value)) = throws.pop_front() {
                // println!("{} to m{}", value, target);
                monkeys[target].items.push_back(value);
            }
        }
    }

    println!("Inspections");
    for i in 0..inspections.len() {
        println!("{}:{}", i, inspections[i]);
    }

    inspections.sort();
    inspections.reverse();
    // println!(
    //     "{} - {}",
    //     inspections[0],
    //     inspections[inspections.len() - 1]
    // );
    Some(inspections[0] * inspections[1])
}

type Throw = (usize, i32);

struct Monkey {
    items: VecDeque<i32>,
    operation: Operation,
    test: i32,
    false_target: usize,
    true_target: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey\n\titem count: {}\n\toperation: {}\n\ttest: {}\n\ttrue -> {} | false -> {}",
            self.items.len(),
            self.operation,
            self.test,
            self.true_target,
            self.false_target
        )
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add(i) => write!(f, "old + {}", i),
            Operation::Multiply(i) => write!(f, "old * {}", i),
            Operation::Power => write!(f, "old * old"),
        }
    }
}
enum Operation {
    Add(i32),
    Multiply(i32),
    Power,
}

impl Monkey {
    fn new(operation: Operation, test: i32, true_target: usize, false_target: usize) -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation,
            test,
            true_target,
            false_target,
        }
    }

    fn throw_next_item(&mut self) -> Throw {
        if let Some(item) = self.items.pop_front() {
            // println!("\nitem:{}", item);
            let mut new_value = match self.operation {
                Operation::Add(x) => item + x,
                Operation::Multiply(x) => item * x,
                Operation::Power => item * item,
            };
            // println!("new value: {}", new_value);
            new_value /= 3;
            // println!("new value div 3: {}", new_value);
            if new_value.clone() % self.test == 0 {
                // println!("thrown to {}", self.true_target);
                return (self.true_target, new_value);
            } else {
                // println!("thrown to {}", self.false_target);
                return (self.false_target, new_value);
            }
        }
        panic!("No items left");
    }
}
