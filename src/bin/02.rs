use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
#[derive(Debug)]
struct Game {
    id: u8,
    rounds: Vec<Round>,
}
fn main() {
    fn parse_color(color_str: &str) -> (Color, u32) {
        let color_vec = color_str.trim().split(" ").collect::<Vec<&str>>();
        let color = match color_vec[1] {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => panic!(),
        };
        return (color, color_vec[0].parse::<u32>().unwrap());
    }
    fn parse_round(rounds: &&str) -> Round {
        let colors: Vec<_> = rounds.split(",").map(parse_color).collect();
        return Round {
            red: colors
                .iter()
                .find(|color| color.0 == Color::Red)
                .map(|color| color.1)
                .unwrap_or(0),
            green: colors
                .iter()
                .find(|color| color.0 == Color::Green)
                .map(|color| color.1)
                .unwrap_or(0),
            blue: colors
                .iter()
                .find(|color| color.0 == Color::Blue)
                .map(|color| color.1)
                .unwrap_or(0),
        };
    }
    fn parse_line(line: &str, _max_cubes: &HashMap<&str, u32>) -> Game {
        let split: Vec<_> = line.split(":").collect();
        let (game, rounds) = (
            split[0].split(" ").last().unwrap().parse::<u8>().unwrap(),
            split[1].split(";").collect::<Vec<&str>>(),
        );
        let parsed_rounds: Vec<Round> = rounds.iter().map(parse_round).collect();
        return Game {
            id: game,
            rounds: parsed_rounds,
        };
    }
    fn check_round(round: &Round, rule_set: &HashMap<&str, u32>) -> bool {
        let Round { red, blue, green } = round;
        if red <= &rule_set["red"] && green <= &rule_set["green"] && blue <= &rule_set["blue"] {
            return true;
        }
        false
    }

    fn check_game(game: &Game, rule_set: &HashMap<&str, u32>) -> Option<u8> {
        let rounds = &game.rounds;
        let possible_rounds = rounds.iter().all(|round| check_round(round, rule_set));
        return if possible_rounds { Some(game.id) } else { None };
    }
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let contents = fs::read_to_string("../input/02.txt").expect("YOU DID SOMETHING WRONG");
    let results: Vec<_> = contents
        .lines()
        .map(|line| parse_line(line, &max_cubes))
        .collect();
    let possible_games: Vec<u8> = results
        .iter()
        .filter_map(|game| check_game(game, &max_cubes))
        .collect();

    println!(
        "{:?}",
        possible_games
            .iter()
            .map(|&game_id| game_id as u32)
            .sum::<u32>()
    );
}
