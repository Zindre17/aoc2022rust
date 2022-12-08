use aoc_helper;
use std::collections::HashSet;
use std::collections::VecDeque;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec6");
    aoc_helper::print_solution(1, |input| partx(4, input), INPUT);
    aoc_helper::print_solution(2, |input| partx(14, input), INPUT);
}

fn partx(find_distinct: usize, input: &str) -> Option<usize> {
    let (first, rest) = input.split_at(find_distinct);
    let mut stream = rest.chars();
    let mut current_window: VecDeque<char> = VecDeque::new();
    let mut current_set = HashSet::<char>::new();

    first.chars().for_each(|i| {
        current_window.push_back(i);
        current_set.insert(i);
    });

    let mut needed = find_distinct;
    while current_set.len() < find_distinct {
        current_set.clear();
        current_window.pop_front();
        current_window.push_back(stream.next().unwrap());

        current_window.iter().for_each(|i| {
            current_set.insert(i.clone());
        });

        needed += 1;
    }

    Some(needed)
}
