use aoc_helper;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec4");
    aoc_helper::print_solution(1, part1, INPUT);
    aoc_helper::print_solution(2, part2, INPUT);
}

fn part1(input: &str) -> Option<i32> {
    let mut fully_contained = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let elves = line
            .split(",")
            .map(|p| {
                let sectors = p
                    .split("-")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                return (sectors[0], sectors[1]);
            })
            .collect::<Vec<(i32, i32)>>();

        let (start1, end1) = elves[0];
        let (start2, end2) = elves[1];

        if (start1 <= start2 && end1 >= end2) || (start1 >= start2 && end1 <= end2) {
            fully_contained += 1;
        }
    }

    Some(fully_contained)
}

fn part2(input: &str) -> Option<i32> {
    let mut fully_contained = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let elves = line
            .split(",")
            .map(|p| {
                let sectors = p
                    .split("-")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                return (sectors[0], sectors[1]);
            })
            .collect::<Vec<(i32, i32)>>();

        let (start1, end1) = elves[0];
        let (start2, end2) = elves[1];

        if (start1 <= start2 && end1 >= start2) || (start1 >= start2 && end2 >= start1) {
            fully_contained += 1;
        }
    }

    Some(fully_contained)
}
