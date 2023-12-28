use std::collections::HashMap;

pub fn solve(input: Vec<String>, args: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    let with_letters = !args.is_empty() && args[0] == "true";

    for line in input {
        let (first_char, last_char) = get_char_numbers(line.clone(), with_letters);
        let value = get_value(first_char, last_char);
        sum += value;

        // println!("{} -> ({}{})", line, first_char, last_char);
    }

    sum
}

fn get_value(v1: char, v2: char) -> i32 {
    let val = format!("{}{}", v1, v2);
    val.parse::<i32>().unwrap_or(0)
}

fn get_char_numbers(line: String, with_letters: bool) -> (char, char) {
    let mut vals: Vec<(usize, char)> = vec![];
    if with_letters {
        let spelled_numbers = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        for (spelled_number, value) in spelled_numbers {
            let mut start_index = 0;
            while let Some(index) = line[start_index..].find(spelled_number) {
                let real_index = start_index + index;
                vals.push((real_index, value));
                start_index = real_index + 1;
            }
        }
    }

    for (i, char) in line.chars().enumerate() {
        match char.to_string().parse::<i32>() {
            Err(_) => {}
            Ok(_) => {
                vals.push((i, char));
            }
        }
    }

    let first_char = vals
        .clone()
        .into_iter()
        .min_by_key(|x| x.0)
        .unwrap_or((0, '0'))
        .1;
    // Last char, otherwise last char is the same as first_char
    let last_char = vals
        .clone()
        .into_iter()
        .max_by_key(|x| x.0)
        .unwrap_or((0, '0'))
        .1;

    (first_char, last_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<String> = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        assert_eq!(solve(input, vec!["false".to_string()]), 142);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<String> = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        assert_eq!(solve(input, vec!["true".to_string()]), 281);
    }

    #[test]
    fn test_special_case() {
        let input = "fivegdsfnfour64sixtqfour";

        let (first_char, last_char) = get_char_numbers(input.to_string(), true);

        assert_eq!(get_value(first_char, last_char), 54)
    }
}
