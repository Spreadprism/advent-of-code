use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum BallType {
    Red,
    Green,
    Blue,
}

struct Game {
    id: i32,
    max_balls: HashMap<BallType, i32>,
}

impl Game {
    fn new(id: i32) -> Game {
        Game {
            id,
            max_balls: HashMap::new(),
        }
    }

    fn from_str(line: String) -> Game {
        todo!()
    }
}
pub fn solve(input: Vec<String>, with_letters: bool) -> i32 {
    let games = load_games(input);

    0
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
        assert_eq!(game.max_balls[&BallType::Red], 3);
        assert_eq!(game.max_balls[&BallType::Green], 20);
        assert_eq!(game.max_balls[&BallType::Blue], 10);
    }
}
