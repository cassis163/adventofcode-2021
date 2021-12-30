use std::fs;

#[path = "../util.rs"]
mod util;

/*
    1. Read each line and remember the last one.
    2. Compare if the current one is more than the last
        a. Increment the count if this is true
    3. Print the count
*/
pub fn solve() {
    let filename = util::get_filename(&"2021", &"1");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let mut lines = contents.lines();
    let mut last_line = lines.next().unwrap();
    let mut count = 0;

    loop {
        match lines.next() {
            Some(current_line) => {
                let last_number = parse_str_to_int32(last_line);
                let current_number = parse_str_to_int32(current_line);

                if current_number > last_number {
                    count = count + 1;
                }

                last_line = current_line;
            },
            None => {
                break;
            }
        }
    }

    println!("Count: {}", count);
}

fn parse_str_to_int32(string_value: &str) -> i32 {
    return string_value.parse().unwrap();
}
