use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: &'static str = include_str!("input.txt");

fn partx(find_distinct: usize) {
    let mut stream = INPUT.chars();
    let mut current_window: VecDeque<char> = VecDeque::new();
    let mut current_set = HashSet::<char>::new();

    while current_window.len() < find_distinct {
        let next = stream.next().unwrap();
        current_window.push_back(next);
        current_set.insert(next);
    }

    let mut needed = find_distinct;
    while current_set.len() < find_distinct {
        current_set.clear();
        current_window.pop_front();
        current_window.push_back(stream.next().unwrap());

        current_window.iter().copied().for_each(|i| {
            current_set.insert(i);
        });

        // println!(
        //     "{}: {}",
        //     current_window.iter().collect::<String>(),
        //     current_set.len()
        // );

        needed += 1;
    }

    println!("{}", needed);
}

fn main() {
    partx(4);
    partx(14);
}
