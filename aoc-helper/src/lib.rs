pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

use std::fmt::Display;
use std::time::Instant;

fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str) {
    let timer = Instant::now();
    let result = func(input);
    let elapsed = timer.elapsed();
    match result {
        Some(result) => {
            println!(
                "({} {}elapsed: {:.2?}){}",
                result, ANSI_ITALIC, elapsed, ANSI_RESET
            );
        }
        None => {
            println!("not solved.")
        }
    }
}

pub fn print_solution<T: Display>(part: usize, func: impl FnOnce(&str) -> Option<T>, input: &str) {
    println!("🎄 {}Part {}{} 🎄", ANSI_BOLD, part, ANSI_RESET);
    print_result(func, input);
}

pub fn print_day(day: &str) {
    println!("💫 {}{}{} 💫", ANSI_BOLD, capitalize(day), ANSI_RESET);
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
