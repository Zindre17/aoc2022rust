use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec9");
    aoc_helper::print_solution(1, part1, INPUT);
    aoc_helper::print_solution(2, part2, INPUT);
}

fn part1(input: &str) -> Option<i32> {
    let mut head: Point = (0, 0);
    let mut tail: Point = (0, 0);
    let mut tail_locations = HashSet::<(i32, i32)>::new();
    tail_locations.insert(tail);
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        let direction = parts[0].chars().next().unwrap();
        let amount = parts[1].parse::<i32>().unwrap();
        for _ in 0..amount {
            head = head.move_in_direction(direction);
            tail = tail.follow(head);
            tail_locations.insert(tail);
        }
    }
    Some(tail_locations.len() as i32)
}

fn part2(input: &str) -> Option<i32> {
    let mut head: Point = (0, 0);
    let mut knots = vec![(0, 0); 8];
    let mut tail: Point = (0, 0);
    let mut tail_locations = HashSet::<(i32, i32)>::new();
    tail_locations.insert(tail);
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        let direction = parts[0].chars().next().unwrap();
        let amount = parts[1].parse::<i32>().unwrap();
        for _ in 0..amount {
            head = head.move_in_direction(direction);
            let mut lead = head;
            for i in 0..knots.len() {
                knots[i] = knots[i].follow(lead);
                lead = knots[i];
            }
            tail = tail.follow(lead);
            tail_locations.insert(tail);
        }
    }
    Some(tail_locations.len() as i32)
}

type Point = (i32, i32);

trait Movable {
    fn move_in_direction(self, dir: char) -> Self;
    fn move_up(self) -> Self;
    fn move_down(self) -> Self;
    fn move_right(self) -> Self;
    fn move_left(self) -> Self;
    fn follow(self, other: Point) -> Self;
}

impl Movable for Point {
    fn move_in_direction(self, dir: char) -> Self {
        match dir {
            'U' => self.move_up(),
            'R' => self.move_right(),
            'D' => self.move_down(),
            'L' => self.move_left(),
            _ => panic!("Unrecognized direction"),
        }
    }

    fn move_up(self) -> Self {
        (self.0, self.1 + 1)
    }

    fn move_right(self) -> Self {
        (self.0 + 1, self.1)
    }

    fn move_down(self) -> Self {
        (self.0, self.1 - 1)
    }

    fn move_left(self) -> Self {
        (self.0 - 1, self.1)
    }

    fn follow(self, other: Point) -> Self {
        let delta_x = other.0 - self.0;
        let delta_y = other.1 - self.1;
        if delta_x.abs() < 2 && delta_y.abs() < 2 {
            return self;
        }
        return (self.0 + delta_x.clamp(-1, 1), self.1 + delta_y.clamp(-1, 1));
    }
}
