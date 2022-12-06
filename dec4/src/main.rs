const INPUT: &'static str = include_str!("input");

fn part1() {
    let mut fully_contained = 0;
    for line in INPUT.lines() {
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

    println!("{}", fully_contained);
}

fn part2() {
    let mut fully_contained = 0;
    for line in INPUT.lines() {
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

    println!("{}", fully_contained);
}

fn main() {
    part1();
    part2();
}
