use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let text = read_lines("src/input/input_01.txt");
    let mut final_number: i32 = 0;
    for line in text {
        // println!("{}", replace_chars(&line));
        let first_rep = replace_chars(&line);
        final_number += &get_calibration(&replace_chars(&first_rep));

    }
    println!("final number: {}", final_number)
}

fn read_lines(filename: &str) -> Vec<String>{
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn replace_chars(input: &str) -> String {
    let re = Regex::new(r#"(one|two|three|four|five|six|seven|eight|nine)"#).unwrap();

    let result = re.replace_all(input, |caps: &regex::Captures| {
        match &caps[0] {
            "one" => "o1e",
            "two" => "t2o",
            "three" => "t3e",
            "four" => "4",
            "five" => "5e",
            "six" => "6",
            "seven" => "7n",
            "eight" => "e8t",
            "nine" => "n9e",
            _ => &caps[0], 
        }
        .to_string()
    });

    result.to_string()
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