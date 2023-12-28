use itertools::Itertools;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum CubeType {
    Red,
    Green,
    Blue,
}

struct Game {
    id: i32,
    max_cubes: HashMap<CubeType, i32>,
}

impl Game {
    fn new(id: i32) -> Game {
        Game {
            id,
            max_cubes: HashMap::new(),
        }
    }

    fn power_cubes(self) -> i32 {
        // multiply all the counts together
        self.max_cubes.iter().fold(1, |acc, (_, count)| acc * count)
    }

    fn from_str(line: String) -> Game {
        let mut parts = line.split(':').collect::<Vec<&str>>();

        let game_id = parts[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        parts[1] = parts[1].strip_prefix(' ').unwrap_or(parts[1]);

        let cube_counts = parts[1].split(' ');

        // iter over each 2 vals

        // for (count, ball_type) in ball_counts.iter().tu
        let mut game = Game::new(game_id);
        for (count_str, cube_str) in cube_counts.into_iter().tuples() {
            let mut count = count_str.parse::<i32>().unwrap();
            let mut stripped = cube_str.strip_suffix(',').unwrap_or(cube_str);
            stripped = cube_str.strip_suffix(';').unwrap_or(stripped);

            let cube_type = match stripped {
                "red" => CubeType::Red,
                "green" => CubeType::Green,
                "blue" => CubeType::Blue,
                _ => panic!("Unknown cube type"),
            };

            count = count.max(*game.max_cubes.get(&cube_type).unwrap_or(&0));

            game.max_cubes.insert(cube_type, count);
        }

        game
    }
}
pub fn solve(input: Vec<String>, args: Vec<String>) -> i32 {
    // remove empty lines
    let filtered_input = input
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    match args[0].as_str() == "true" {
        true => solve_day_2(filtered_input),
        false => solve_day_1(filtered_input, args[1..].to_vec()),
    }
}

fn solve_day_1(input: Vec<String>, args: Vec<String>) -> i32 {
    let games = load_games(input);

    let red_cubes = args[0].parse::<i32>().unwrap();
    let green_cubes = args[1].parse::<i32>().unwrap();
    let blue_cubes = args[2].parse::<i32>().unwrap();

    let mut total = 0;

    for game in games {
        let mut possible = true;

        if game.max_cubes[&CubeType::Green] > green_cubes {
            possible = false;
        }

        if game.max_cubes[&CubeType::Blue] > blue_cubes {
            possible = false;
        }

        if game.max_cubes[&CubeType::Red] > red_cubes {
            possible = false;
        }

        if possible {
            total += game.id;
        }
    }

    total
}

fn solve_day_2(input: Vec<String>) -> i32 {
    let games = load_games(input);
    games.into_iter().map(|game| game.power_cubes()).sum()
}

fn load_games(input: Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    for line in input {
        games.push(Game::from_str(line));
    }

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let test = "Game 13: 6 green, 1 red, 1 blue; 10 blue, 15 green; 1 blue, 2 red, 5 green; 2 blue, 1 red, 20 green; 3 blue, 3 red, 10 green";

        let game = Game::from_str(test.to_string());

        assert_eq!(game.id, 13);
        assert_eq!(game.max_cubes[&CubeType::Red], 3);
        assert_eq!(game.max_cubes[&CubeType::Green], 20);
        assert_eq!(game.max_cubes[&CubeType::Blue], 10);
    }

    #[test]
    fn test_part_1() {
        let test = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];

        let answer = solve(
            test,
            vec!["12".to_string(), "13".to_string(), "14".to_string()],
        );

        assert_eq!(answer, 8);
    }
}
