const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Dec8");
    aoc_helper::print_solution(1, part1, INPUT);
    aoc_helper::print_solution(2, part2, INPUT);
}

fn part1(input: &str) -> Option<i32> {
    let (trees, columns, rows) = get_tree_map(input);

    let mut visible_trees = 0;
    for (index, height) in trees.iter().enumerate() {
        if is_edge_tree(index, rows, columns) {
            visible_trees += 1;
            continue;
        }
        if is_visible_down(&trees, columns as i32, index as i32, *height) {
            visible_trees += 1;
            continue;
        }
        if is_visible_left(&trees, columns as i32, index as i32, *height) {
            visible_trees += 1;
            continue;
        }
        if is_visible_right(&trees, columns as i32, index as i32, *height) {
            visible_trees += 1;
            continue;
        }
        if is_visible_up(&trees, columns as i32, index as i32, *height) {
            visible_trees += 1;
        }
    }

    Some(visible_trees)
}

fn part2(input: &str) -> Option<i32> {
    let (trees, columns, rows) = get_tree_map(input);
    let mut best_score = 0;

    for (index, height) in trees.iter().enumerate() {
        if is_edge_tree(index, rows, columns) {
            continue;
        }
        let mut current_score = 1;
        current_score *= distance_down(&trees, columns as i32, index as i32, *height);
        current_score *= distance_right(&trees, columns as i32, index as i32, *height);
        current_score *= distance_up(&trees, columns as i32, index as i32, *height);
        current_score *= distance_left(&trees, columns as i32, index as i32, *height);
        if current_score > best_score {
            best_score = current_score;
        }
    }

    Some(best_score)
}

fn get_tree_map(input: &str) -> (Vec<i32>, usize, usize) {
    let trees = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let columns = input.lines().next().unwrap().len();
    let rows = trees.len() / columns;
    (trees, columns, rows)
}

fn is_edge_tree(index: usize, width: usize, height: usize) -> bool {
    let is_first_row = index < width;
    let is_last_row = index > width * height - width;
    let is_first_column = index % width == 0;
    let is_last_column = index % width == width - 1;
    return is_first_column || is_first_row || is_last_column || is_last_row;
}

fn is_visible_up(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> bool {
    let mut current_index = index - width;
    while current_index > 0 {
        if items[current_index as usize] >= tree_height {
            return false;
        };
        current_index -= width;
    }
    true
}

fn is_visible_down(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> bool {
    let mut current_index = index + width;
    while current_index < items.len() as i32 {
        if items[current_index as usize] >= tree_height {
            return false;
        };
        current_index += width;
    }
    true
}

fn is_visible_right(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> bool {
    let mut current_index = index + 1;
    while current_index % width > 0 {
        if items[current_index as usize] >= tree_height {
            return false;
        };
        current_index += 1;
    }
    true
}

fn is_visible_left(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> bool {
    let mut current_index = index - 1;
    for _ in 0..(index % width) {
        if items[current_index as usize] >= tree_height {
            return false;
        };
        current_index -= 1;
    }
    true
}

fn distance_up(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> i32 {
    let mut current_index = index - width;
    let mut distance = 1;
    while current_index > 0 {
        if items[current_index as usize] >= tree_height {
            return distance;
        };
        current_index -= width;
        distance += 1;
    }
    distance - 1
}

fn distance_down(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> i32 {
    let mut current_index = index + width;
    let mut distance = 1;
    while current_index < items.len() as i32 {
        if items[current_index as usize] >= tree_height {
            return distance;
        };
        current_index += width;
        distance += 1;
    }
    distance - 1
}

fn distance_right(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> i32 {
    let mut current_index = index + 1;
    let mut distance = 1;
    while current_index % width > 0 {
        if items[current_index as usize] >= tree_height {
            return distance;
        };
        current_index += 1;
        distance += 1;
    }
    distance - 1
}

fn distance_left(items: &Vec<i32>, width: i32, index: i32, tree_height: i32) -> i32 {
    let mut current_index = index - 1;
    let mut distance = 1;
    for _ in 0..(index % width) {
        if items[current_index as usize] >= tree_height {
            return distance;
        };
        current_index -= 1;
        distance += 1;
    }
    distance - 1
}
