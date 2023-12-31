mod utils;

use utils::read_lines;

fn main() {
    let text = read_lines("src/input/input_01.txt");
    let mut final_number: i32 = 0;
    for line in text {
        final_number += &get_calibration(&line);
        // println!("{}", get_calibration(&line))
    }
    println!("final number: {}", final_number)
}

fn get_calibration(line: &str) -> i32 {
    let mut num_chars = vec![];

    for c in line.chars() {
        if c.is_numeric() {
            num_chars.push(c);
        }
    }
    let mut calibration: String = num_chars[0].clone().to_string();
    calibration.push_str(&num_chars[num_chars.len() - 1].to_string());
    calibration.parse::<i32>().unwrap()
}
