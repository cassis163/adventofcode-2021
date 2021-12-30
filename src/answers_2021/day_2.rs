use std::fs;

#[path = "../util.rs"]
mod util;

/*
    1. Iterate through lines
        a. Update position
    2. Print multiplication of horizontal and position and depth
*/
pub fn solve() {
    let filename = util::get_filename(&"2021", &"2");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let mut distance = 0;
    let mut depth = 0;

    contents.lines().for_each(|line| {
        if line.starts_with("down") {
            depth = depth + get_number(line.strip_prefix("down ").unwrap());
        } else if line.starts_with("up") {
            depth = depth - get_number(line.strip_prefix("up ").unwrap());
        } else if line.starts_with("forward") {
            distance = distance + get_number(line.strip_prefix("forward ").unwrap());
        }
    });

    println!("{}", distance * depth);
}

fn get_number(str_number: &str) -> i32 {
    return str_number.parse().expect("Failed to parse string!")
}
