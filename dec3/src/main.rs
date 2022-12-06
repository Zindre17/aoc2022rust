const INPUT: &'static str = include_str!("input");

fn part1() {
    let start = 'a' as i32 - 1;

    let mut sum = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }
        let half = line.len() / 2;
        let firstHalf = line.get(..half);
        let secondHalf = line.get(half..);

        let mut letterInBoth = None;
        for letter in firstHalf.unwrap().chars() {
            if secondHalf.unwrap().contains(letter) {
                letter.is_uppercase().then(|| sum += 26);
                letterInBoth = Some(letter.to_lowercase().collect::<Vec<char>>()[0]);
                break;
            }
        }
        sum += letterInBoth.unwrap() as i32 - start;
    }

    println!("{}", sum);
}

fn part2() {
    let start = 'a' as i32 - 1;

    let mut sum = 0;
    let mut lines = INPUT.lines();

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
    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
