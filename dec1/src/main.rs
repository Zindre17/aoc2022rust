use std::{fs::File, io::Read, path::Path};

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Count not read {}: {}", display, why),
        Ok(_) => return s,
    }
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let content = read_file("input");
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
        println!("{}", elf);
        sum += elf;
    }
    println!("{}", sum);
}

fn part1() {
    let content = read_file("input");

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
