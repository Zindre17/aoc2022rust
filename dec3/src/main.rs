use aoc_helper;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec3");
    aoc_helper::print_solution(1, part1, INPUT);
    aoc_helper::print_solution(2, part2, INPUT);
}

fn part1(input: &str) -> Option<i32> {
    let start = 'a' as i32 - 1;

    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let half = line.len() / 2;
        let first_half = line.get(..half);
        let second_half = line.get(half..);

        for letter in first_half.unwrap().chars() {
            if second_half.unwrap().contains(letter) {
                letter.is_uppercase().then(|| sum += 26);
                sum += letter.to_lowercase().collect::<Vec<char>>()[0] as i32 - start;
                break;
            }
        }
    }

    Some(sum)
}

fn part2(input: &str) -> Option<i32> {
    let start = 'a' as i32 - 1;

    let mut sum = 0;
    let mut lines = input.lines();

    let mut line = lines.next();
    while line.is_some() {
        let l2 = lines.next();
        let l3 = lines.next();
        for letter in line.unwrap().chars() {
            if l2.unwrap().contains(letter) && l3.unwrap().contains(letter) {
                letter.is_uppercase().then(|| sum += 26);
                sum += letter.to_lowercase().collect::<Vec<char>>()[0] as i32 - start;
                break;
            }
        }
        line = lines.next();
    }
    Some(sum)
}
