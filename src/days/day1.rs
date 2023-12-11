pub fn solve(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for line in input {
        let mut all_numbers: String = "".to_string();
        for char in line.chars() {
            match char.to_string().parse::<i32>() {
                Err(_) => {}
                Ok(_) => {
                    all_numbers.push(char);
                }
            }
        }
        let first_char = all_numbers.chars().next().unwrap_or('0');
        // Last char, otherwise last char is the same as first_char
        let last_char = all_numbers.chars().last().unwrap_or(first_char);

        let number: i32 = format!("{}{}", first_char, last_char).parse().unwrap();
        sum += number;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input: Vec<String> = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        assert_eq!(solve(input), 142);
    }
}
