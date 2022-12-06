const INPUT: &'static str = include_str!("input.txt");

fn part1() {
    // find initial state
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines = INPUT.lines();
    let mut line = lines.next();

    while !line.unwrap().is_empty() {
        let line_chars = line.unwrap().chars().collect::<Vec<char>>();

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

        line = lines.next();
    }

    stacks
        .iter()
        .map(|i| i.iter().collect::<String>())
        .for_each(|i| println!("{}", i));

    stacks.iter_mut().for_each(|stack| stack.reverse());

    // read move instructions
    line = lines.next();
    while !line.unwrap().is_empty() {
        let instruction_parts = line.unwrap().split(" ").collect::<Vec<&str>>();
        let amount = instruction_parts[1].parse::<i32>().unwrap();
        let origin = instruction_parts[3].parse::<i32>().unwrap();
        let destination = instruction_parts[5].parse::<i32>().unwrap();

        println!("moving {} from {} to {}", amount, origin, destination);
        (0..amount).for_each(|_| {
            let moved = stacks[origin as usize].pop();
            stacks[destination as usize].push(moved.unwrap());
        });

        line = lines.next();
    }

    println!(
        "{}",
        stacks
            .iter()
            .map(|stack| match stack.last() {
                Some(x) => x.to_string(),
                None => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    );
}

fn part2() {
    // find initial state
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines = INPUT.lines();
    let mut line = lines.next();

    while !line.unwrap().is_empty() {
        let line_chars = line.unwrap().chars().collect::<Vec<char>>();

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

        line = lines.next();
    }

    stacks
        .iter()
        .map(|i| i.iter().collect::<String>())
        .for_each(|i| println!("{}", i));

    stacks.iter_mut().for_each(|stack| stack.reverse());

    // read move instructions
    line = lines.next();
    while !line.unwrap().is_empty() {
        let instruction_parts = line.unwrap().split(" ").collect::<Vec<&str>>();
        let amount = instruction_parts[1].parse::<i32>().unwrap();
        let origin = instruction_parts[3].parse::<i32>().unwrap();
        let destination = instruction_parts[5].parse::<i32>().unwrap();

        println!("moving {} from {} to {}", amount, origin, destination);

        let stack_length = stacks[origin as usize].len();

        println!(
            "current state of origin: {}",
            stacks[origin as usize].iter().collect::<String>()
        );

        let mut to_be_moved = stacks[origin as usize]
            .splice((stack_length - amount as usize).., Vec::new())
            .collect::<Vec<char>>();

        println!("removed: {}", to_be_moved.iter().collect::<String>());

        stacks[destination as usize].append(&mut to_be_moved);
        line = lines.next();
    }

    println!(
        "{}",
        stacks
            .iter()
            .map(|stack| match stack.last() {
                Some(x) => x.to_string(),
                None => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    );
}

fn main() {
    part1();
    part2();
}
