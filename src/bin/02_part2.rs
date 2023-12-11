use std::fs;

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
    fn parse_line(line: &str) -> Game {
        let split: Vec<_> = line.split(":").collect();
        let (_game, rounds) = (
            split[0].split(" ").last().unwrap().parse::<u8>().unwrap(),
            split[1].split(";").collect::<Vec<&str>>(),
        );
        let parsed_rounds: Vec<Round> = rounds.iter().map(parse_round).collect();
        return Game {
            rounds: parsed_rounds,
        };
    }

    fn check_game(game: &Game) -> u32 {
        let rounds = &game.rounds;
        let min_red = rounds.iter().map(|round| round.red).max().unwrap_or(0);
        let min_green = rounds.iter().map(|round| round.green).max().unwrap_or(0);
        let min_blue = rounds.iter().map(|round| round.blue).max().unwrap_or(0);
        min_red * min_green * min_blue
    }
    let contents = fs::read_to_string("../input/02.txt").expect("YOU DID SOMETHING WRONG");
    let results: Vec<_> = contents.lines().map(|line| parse_line(line)).collect();
    let lowest_cubes: Vec<u32> = results.iter().map(|game| check_game(game)).collect();

    println!("{:?}", lowest_cubes);
    println!("{}", lowest_cubes.iter().sum::<u32>());
}
