mod days;
mod utilities;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day_number_str = &args[1];
    // remove the leading arg
    //
    println!("Running advent-of-code day {}", day_number_str);

    let day_number: i32 = day_number_str.parse().unwrap();

    let input = utilities::read_files(day_number);
    let solve_args = args[2..].to_vec();

    let output = match day_number {
        1 => Some(days::day1::solve(input, solve_args)),
        2 => Some(days::day2::solve(input, solve_args)),
        _ => None,
    };

    match output {
        Some(result) => println!("------------------\n{}", result),
        None => println!("Day {} not implemented yet", day_number),
    }
}
