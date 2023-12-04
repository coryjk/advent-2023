use std::collections::HashMap;
use crate::problem::Problem;

pub struct Day02 {
    pub input: Vec<String>,
}

#[derive(Clone)]
struct Cubes {
    pub color: String,
    pub quantity: u32,
}

#[derive(Clone)]
struct Draw {
    pub cubes: Vec<Cubes>,
}

impl Draw {

    fn parse_draw(line: &str) -> Draw {
        let cubes = line.split(",")
            // expecting format: "{num} {color}"
            .map(|split| {
                let count_and_color = split.split(" ")
                    .filter(|str| !str.is_empty());
                Cubes {
                    quantity: count_and_color.to_owned().into_iter()
                        .nth(0)
                        .and_then(|q| q.parse::<u32>().ok())
                        .unwrap(),
                    color: count_and_color.to_owned().into_iter()
                        .nth(1)
                        .expect("Could not extract color!")
                        .to_string(),
                }
            })
            .collect::<Vec<Cubes>>();

        Draw { cubes }
    }
}

#[derive(Clone)]
struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl Game {

    fn create_game(line: &str) -> Game {
        // expecting below format:
        // "Game 79: 3 green, 1 blue, 2 red; 8 green, 1 blue, 2 red; 2 blue, 1 red, 11 green"
        let split = line.split(":");
        let id = split.to_owned().into_iter().nth(0)
            .map(|left|
                left.split(" ")
                    .nth(1)
                    .expect("Could not parse game ID!")
            )
            .map(|num| num.parse::<u32>().unwrap())
            .unwrap_or_else(|| panic!("Could not extract game ID from line: {line}"));

        // expecting below format from here:
        // "3 green, 1 blue, 2 red; 8 green, 1 blue, 2 red; 2 blue, 1 red, 11 green"
        let draws = split.to_owned().into_iter().nth(1)
            .map(|d| d.split(";"))
            // ["3 green, 1 blue, 2 red", "8 green, 1 blue, 2 red", ...]
            .map(|split| split
                .map(|draw| Draw::parse_draw(draw))
                .collect::<Vec<Draw>>()
            )
            .unwrap_or_else(|| panic!("Could not extract draws from line: {line}"));

        Game { id, draws }
    }

    fn is_valid(game: &Game, constraints: &HashMap<&str, u32>) -> bool {
        game.draws.iter()
            .map(|draw|
                draw.cubes.iter()
                    .all(|cube| cube.quantity <= *constraints.get(&*cube.color).unwrap())
            )
            .all(|is_valid_draw| is_valid_draw)
    }

    fn min_constraints(game: &Game) -> HashMap<String, u32> {
        let mut min_constraints: HashMap<String, u32> = HashMap::new();

        // min constraints is the max draw present for a given color
        game.draws.iter()
            .flat_map(|draw| draw.cubes.iter())
            .for_each(|cubes| {
                let current_constraint = *min_constraints.get(&*cubes.color).unwrap_or(&0);
                if cubes.quantity > current_constraint {
                    min_constraints.insert(cubes.color.clone(), cubes.quantity);
                }
            });

        min_constraints
    }
}

impl Problem for Day02 {
    fn solve_part_one(&self) -> String {
        let constraints: HashMap<&str, u32> = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);

        self.input.iter()
            .map(|line| Game::create_game(line))
            .filter(|game| Game::is_valid(game, &constraints))
            .map(|valid_game| valid_game.id)
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_two(&self) -> String {
        self.input.iter()
            .map(|line| Game::create_game(line).to_owned())
            .map(|game| Game::min_constraints(&game))
            .map(|constraints|
                constraints.values()
                    .map(|n| *n)
                    .reduce(|x,y| x * y)
                    .unwrap()
            )
            .sum::<u32>()
            .to_string()
    }
}