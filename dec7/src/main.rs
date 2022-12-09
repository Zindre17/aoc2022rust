use aoc_helper;
use std::collections::HashMap;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    aoc_helper::print_day("Day7");
    aoc_helper::print_solution(1, part1_3, INPUT);
}

// fn part1(input: &str) -> Option<i32> {
//     use Item::*;

//     const max: i32 = 100_000;

//     let mut current = "/";
//     let mut items = HashMap::new();

//     for line in input.lines() {
//         if line.is_empty() {
//             break;
//         }
//         match line {
//             l if l.starts_with("$ cd") => {
//                 let folder = l.get(4..).unwrap();
//                 match items.get_mut(folder) {
//                     None => {
//                         items.insert(folder, Dir(Vec::new()));
//                         items.get_mut(folder).unwrap();
//                     }
//                     _ => {}
//                 }
//                 current = folder;
//             }
//             l if l.starts_with("$ ls") => {}
//             l => {
//                 let splits = l.split(' ').collect::<Vec<&str>>();
//                 let (item_type, item_name) = (splits[0], splits[1]);
//                 let item = match items.get_mut(item_name) {
//                     Some(i) => i,
//                     None => match item_type {
//                         "dir" => {
//                             items.insert(item_name, Dir(Vec::new()));
//                             items.get_mut(item_name).unwrap()
//                         }
//                         _ => {
//                             items.insert(item_name, File(item_type.parse::<i32>().unwrap()));
//                             items.get_mut(item_name).unwrap()
//                         }
//                     },
//                 };
//                 if let Dir(sub_items) = items.get_mut(current).unwrap() {
//                     sub_items.push(item);
//                 }
//                 // let item: &Item = match item_type {
//                 //     "dir" => match items.get(item_name) {
//                 //         None => {
//                 //             items.insert(item_name, Dir(Vec::new()));
//                 //             items.get(item_name).unwrap()
//                 //         }
//                 //         Some(i) => i,
//                 //     },
//                 //     _ => {
//                 //         &mut items.insert(item_name, File(item_type.parse::<i32>().unwrap()));
//                 //         &mut items.get(item_name).unwrap()
//                 //     }
//                 // };
//                 // match current {
//                 //     Dir(mut dir_items) => dir_items.push(item),
//                 //     _ => unreachable!(),
//                 // };
//             }
//         }
//     }

//     // fn get_or_add_dir<'map>(
//     //     map: &'map mut HashMap<&'map str, Item>,
//     //     name: &'map str,
//     // ) -> &'map Item {
//     //     if let Some(item) = map.get(name) {
//     //         item
//     //     } else {
//     //         map.insert(name, Item::Dir(Vec::new())).unwrap();
//     //         return map.get(name).unwrap();
//     //     }
//     // }

//     Some(1)
// }

// fn part1_2(input: &str) -> Option<i32> {
//     const max: i32 = 100_000;

//     let mut root = Box::new(Directory {
//         name: "/".to_string(),
//         parent: None,
//         content: HashMap::new(),
//     });

//     let mut current: &Directory = &root;

//     for line in input.lines() {
//         if line.is_empty() {
//             break;
//         }
//         match line {
//             l if l.starts_with("$ cd") => {
//                 let target = l.get(4..).unwrap();
//                 let next: &Directory = match target {
//                     "/" => &current,
//                     ".." => &current.parent.as.unwrap(),
//                     _ => {
//                         // {
//                         //     let mut new_dir = Directory {
//                         //         name: target.to_string(),
//                         //         parent: Some(&mut current),
//                         //         content: HashMap::new(),
//                         //     };
//                         // }
//                         // current
//                         //     .content
//                         //     .entry(target.to_string())
//                         //     .or_insert(Item::Dir(new_dir));
//                         &mut current
//                     }
//                 };
//                 current = next;
//             }
//             l if l.starts_with("$ ls") => {}
//             l => {
//                 // let splits = l.split(' ').collect::<Vec<&str>>();
//                 // let (item_type, item_name) = (splits[0], splits[1]);
//                 // let item = match item_type {
//                 //     "dir" => Item::Dir(Directory {
//                 //         name: item_name.to_string(),
//                 //         parent: Some(&current),
//                 //         content: HashMap::new(),
//                 //     }),
//                 //     _ => Item::File(File {
//                 //         name: item_name.to_string(),
//                 //         size: item_type.parse::<i32>().unwrap(),
//                 //     }),
//                 // };
//                 // current.content.insert(item_name.to_string(), item);
//             }
//         }
//     }

//     Some(1)
// }

fn part1_3(input: &str) -> Option<i32> {
    let mut folder_sizes = HashMap::<String, i32>::new();
    let mut current_path = "/".to_owned();
    for line in input.lines().skip(1) {
        if line.is_empty() {
            break;
        }
        match line {
            _ if line.starts_with("$ cd") => {
                let target = line.get(5..).unwrap();
                match target {
                    _ if target.starts_with("/") => {}
                    _ if target.starts_with("..") => {
                        let mut parts = current_path.split("/").collect::<Vec<&str>>();
                        parts.pop();
                        let mut parent_path = parts.join("/");
                        if parent_path.is_empty() {
                            parent_path = "/".to_string();
                        }
                        let size: i32;
                        {
                            size = *folder_sizes.get(&current_path.clone()).unwrap();
                        }
                        folder_sizes.entry(parent_path.clone()).and_modify(|s| {
                            *s += size;
                        });
                        current_path = parent_path;
                    }
                    _ => {
                        if current_path != "/" {
                            current_path.push_str("/");
                        }
                        current_path.push_str(target);
                    }
                };
                folder_sizes.entry(current_path.to_owned()).or_insert(0);
            }
            _ if line.starts_with("$ ls") => {}
            _ => {
                let splits = line.split(' ').collect::<Vec<&str>>();
                let (item_type, item_name) = (splits[0], splits[1]);
                match item_type {
                    "dir" => {
                        let new_path = &(current_path.clone() + item_name);
                        folder_sizes.entry(new_path.clone()).or_insert(0);
                    }
                    _ => {
                        folder_sizes.entry(current_path.clone()).and_modify(|c| {
                            *c += item_type.parse::<i32>().unwrap();
                        });
                    }
                };
            }
        }
    }
    while current_path != "/" {
        let mut parts = current_path.split("/").collect::<Vec<&str>>();
        parts.pop();
        let mut parent_path = parts.join("/");
        if parent_path.is_empty() {
            parent_path = "/".to_string();
        }
        let size: i32;
        {
            size = *folder_sizes.get(&current_path.clone()).unwrap();
        }
        folder_sizes.entry(parent_path.clone()).and_modify(|s| {
            *s += size;
        });

        current_path = parent_path;
    }
    let mut sum = 0;
    const MAX: i32 = 100_000;
    for (_, value) in folder_sizes {
        if value < MAX {
            sum += value;
        }
    }

    Some(sum)
}

// struct Directory {
//     name: String,
//     parent: Option<Box<Directory>>,
//     content: HashMap<String, Item>,
// }

// struct File {
//     name: String,
//     size: i32,
// }

// enum Item {
//     File(File),
//     Dir(Directory),
// }

// enum Item<'a> {
//     File(i32),
//     Dir(Vec<&'a Item<'a>>),
// }
