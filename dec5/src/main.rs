use aoc_helper;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec5");
    aoc_helper::print_solution(1, part1, INPUT);
    aoc_helper::print_solution(2, part2, INPUT);
}

fn part1(input: &str) -> Option<String> {
    // find initial state
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let line_chars = line.chars().collect::<Vec<char>>();

        let mut char_index = 1;
        let mut stack_index = 1;
        while char_index < line_chars.len() {
            if line_chars[char_index].is_numeric() {
                break;
            }
            if !line_chars[char_index].is_whitespace() {
                while stack_index >= stacks.len() {
                    stacks.push(Vec::new());
                }
                stacks[stack_index].push(line_chars[char_index]);
            }
            char_index += 4;
            stack_index += 1;
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());

    // read move instructions
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let instruction_parts = line.split(" ").collect::<Vec<&str>>();
        let amount = instruction_parts[1].parse::<i32>().unwrap();
        let origin = instruction_parts[3].parse::<i32>().unwrap();
        let destination = instruction_parts[5].parse::<i32>().unwrap();

        (0..amount).for_each(|_| {
            let moved = stacks[origin as usize].pop();
            stacks[destination as usize].push(moved.unwrap());
        });
    }

    Some(
        stacks
            .iter()
            .map(|stack| match stack.last() {
                Some(x) => x.to_string(),
                None => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join(""),
    )
}

fn part2(input: &str) -> Option<String> {
    // find initial state
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let line_chars = line.chars().collect::<Vec<char>>();

        let mut char_index = 1;
        let mut stack_index = 1;
        while char_index < line_chars.len() {
            if line_chars[char_index].is_numeric() {
                break;
            }
            if !line_chars[char_index].is_whitespace() {
                while stack_index >= stacks.len() {
                    stacks.push(Vec::new());
                }
                stacks[stack_index].push(line_chars[char_index]);
            }
            char_index += 4;
            stack_index += 1;
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());

    // read move instructions
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let instruction_parts = line.split(" ").collect::<Vec<&str>>();
        let amount = instruction_parts[1].parse::<i32>().unwrap();
        let origin = instruction_parts[3].parse::<i32>().unwrap();
        let destination = instruction_parts[5].parse::<i32>().unwrap();

        let stack_length = stacks[origin as usize].len();

        let mut to_be_moved = stacks[origin as usize]
            .splice((stack_length - amount as usize).., Vec::new())
            .collect::<Vec<char>>();

        stacks[destination as usize].append(&mut to_be_moved);
    }

    Some(
        stacks
            .iter()
            .map(|stack| match stack.last() {
                Some(x) => x.to_string(),
                None => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join(""),
    )
}
