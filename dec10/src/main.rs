const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec10");
    aoc_helper::print_solution(1, part1, INPUT);
    aoc_helper::print_solution(2, part2, INPUT);
}

fn part1(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut value = 1;
    let mut clock = 1;
    let mut signal_sum = 0;
    let mut tick = |value| {
        clock += 1;
        if (clock - 20) % 40 == 0 {
            signal_sum += clock * value;
        }
    };

    for line in lines {
        if line.is_empty() {
            break;
        }
        match line {
            "noop" => tick(value),
            _ => {
                tick(value);
                value += line
                    .split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                tick(value);
            }
        }
    }
    Some(signal_sum)
}

fn part2(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut value = 1;
    let mut clock = 1;
    let mut row = 0;
    let mut pixel_line = vec!['.'; 40];
    let mut tick = |value: i32| {
        clock += 1;
        let pixel = (clock - 1) % 40;
        if clock % 40 == 0 {
            row = (clock - 20) / 40;
            println!("{}", String::from_iter(&pixel_line));
        }
        if (pixel - value).abs() < 2 {
            pixel_line[(pixel) as usize] = '#';
        } else {
            pixel_line[(pixel) as usize] = '.';
        }
    };

    for line in lines {
        if line.is_empty() {
            break;
        }
        match line {
            "noop" => tick(value),
            _ => {
                tick(value);
                value += line
                    .split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                tick(value);
            }
        }
    }
    Some(1)
}
