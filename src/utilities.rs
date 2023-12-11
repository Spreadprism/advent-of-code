use std::fs;

pub fn read_files(day_number: i32) -> Vec<String> {
    let file_path = format!("inputs/day/{}/input", day_number);
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    lines
}
