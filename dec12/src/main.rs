use std::collections::HashMap;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec12");
    aoc_helper::print_solution(1, part1, INPUT);
}

fn part1(input: &str) -> Option<i32> {
    let mut map = generate_altitude_map(input);

    let current_position = map.get_starting_position();
    let target_position = map.get_target_position();

    traverse_map(&mut map);

    Some(target_position as i32)
}

fn traverse_map(map: &mut AltitudeMap) {
    let start = map.get_starting_position();

    let ref mut to_check = Vec::<(usize, usize)>::new();

    let mut graph =
        HashMap::<usize, i32>::from_iter(map.map.iter().enumerate().map(|(i, _)| (i, i32::MAX)));

    graph.insert(start, 0);
    to_check.push(start);

    while let Some(next) = to_check.pop() {
        let current = *map.map.get(next).unwrap();

        let up_index = next - map.width as usize;
        if let Some(&up) = graph.get(&up_index) {
            if check_neighbour(map, current, up_index) {
                if up < (current + 1) {
                    graph.insert(up_index, current + 1);
                    to_check.push(up_index);
                }
            }
        }
        let down_index = next + map.width as usize;
        if let Some(down) = graph.get(&down_index) {
            if check_neighbour(map, current, down_index) {
                graph.insert(down_index, current + 1);
                to_check.push(down_index);
            }
        }
        let left_index = next - 1;
        if let Some(left) = graph.get(&left_index) {
            if check_neighbour(map, current, left_index) {
                graph.insert(left_index, current + 1);
                to_check.push(left_index);
            }
        }
        let right_index = next + 1;
        if let Some(right) = graph.get(&(next + 1)) {
            if check_neighbour(map, current, right_index) {
                graph.insert(right_index, current + 1);
                to_check.push(right_index);
            }
        }
    }
}

fn check_neighbour(map: &AltitudeMap, current_altitude: i32, index: usize) -> bool {
    let altitude = *map.map.get(index).unwrap();
    return altitude > current_altitude && altitude - current_altitude < 2;
}

fn generate_altitude_map(input: &str) -> AltitudeMap {
    let mut lines = input.lines();
    let line = lines.next().unwrap();
    let width = line.len();
    let mut alt_map = Vec::<i32>::new();
    for c in line.chars() {
        alt_map.push(c as i32);
    }
    for line in lines {
        for c in line.chars() {
            alt_map.push(c as i32);
        }
    }
    AltitudeMap {
        map: alt_map,
        width: width as i32,
    }
}

struct Node<'a> {
    parent: Option<&'a mut Node<'a>>,
    out: Vec<Node<'a>>,
    distance: i32,
}

impl<'a> Node<'a> {
    fn new() -> Node<'a> {
        Node {
            parent: None,
            distance: 0,
            out: Vec::new(),
        }
    }

    // fn create_next(&'a mut self) -> &'a Node<'a> {
    //     // let node = Node {
    //     //     parent: Some(self),
    //     //     distance: self.distance + 1,
    //     //     out: Vec::new(),
    //     // };
    //     // self.out.push(&node);
    //     // return &node;
    // }
}

struct AltitudeMap {
    map: Vec<i32>,
    width: i32,
}

impl AltitudeMap {
    fn get_starting_position(&self) -> usize {
        self.map.binary_search(&('S' as i32)).unwrap()
    }

    fn get_target_position(&self) -> usize {
        self.map.binary_search(&('E' as i32)).unwrap()
    }
}
