const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec1");
    aoc_helper::print_solution(1, part1, INPUT);
    aoc_helper::print_solution(2, part2, INPUT);
}

fn part2(input: &str) -> Option<i32> {
    let mut elves = Vec::new();

    let mut sum = 0;

    let lines = input.lines();
    for line in lines {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    elves.sort();
    elves.reverse();
    sum = 0;
    for elf in elves.get(0..3).unwrap() {
        sum += elf;
    }

    return Some(sum);
}

fn part1(input: &str) -> Option<i32> {
    let mut max = 0;
    let mut sum = 0;

    let lines = input.lines();

    for line in lines {
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    Some(max)
}
