use input_reader;

fn main() {
    part1();
    part2();
}

fn part2() {
    let content = input_reader::read_file("input");
    let mut elves = vec![1i32, 0, 10];

    let mut sum = 0;

    let lines = content.split("\r\n");
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

    println!("{}", sum);
}

fn part1() {
    let content = input_reader::read_file("input");

    let mut max = 0;
    let mut sum = 0;

    let lines = content.split("\r\n");

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

    println!("\n\n{}", max);
}
