pub fn get_filename(year: &str, day: &str) -> String {
    return format!("./data/{}/day_{}.txt", year, day);
}
