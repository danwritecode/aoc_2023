use itertools::Itertools;

pub fn part_one() -> i32 {
    let input = include_str!("../input");
    let lines = input.lines().collect::<Vec<&str>>();

    let sum = lines
        .into_iter()
        .map(|l| {
            let game = Game::new(l);
            game.calc_winning_total()
        })
        .sum::<i32>();

    sum
}

pub fn part_two() -> i32 {
    let input = include_str!("../input");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut counts = vec![1; lines.len()];

    for (li, l) in lines.iter().enumerate() {
        let game = Game::new(l);
        let matches = game.calc_matches();
        let ct = counts[li];

        for i in 0..matches {
            counts[i + li + 1] += ct;
        }
    }

    counts.iter().sum::<i32>()
}



struct Game {
    id: i32,
    winning: Vec<i32>,
    picked: Vec<i32>
}

impl Game {
    fn new(game: &str) -> Self {
        let parts = game.split(':').collect::<Vec<&str>>();
        let id = Game::parse_game_id(parts[0]);
        let (winning, picked) = Game::parse_game(parts[1]);

        Game {
            id,
            winning,
            picked,
        }
    }

    fn calc_winning_total(&self) -> i32 {
        let matching = &self.calc_matches();

        if *matching == 0 {
            return 0;
        }

        let base: i32 = 2;
        let total = base.pow(*matching as u32 - 1);
        println!("total game {} = {}", &self.id, total);

        total
    }

    fn calc_matches(&self) -> usize {
        self.picked
            .iter()
            .map(|p| {
                if self.winning.contains(p) {
                    return 1;
                }
                return 0;
            })
            .sum::<usize>()
    }


    fn parse_game_id(label: &str) -> i32 {
        let ws_index = label.rfind(" ").unwrap() + 1;
        let id = &label[ws_index..];
        id.parse::<i32>().expect("Failed to parse game id to i32")
    }

    fn parse_game(game: &str) -> (Vec<i32>, Vec<i32>) {
        let game = game.trim();
        let (winning, picked) = game.split('|').map(|s| s.trim()).collect_tuple::<(&str, &str)>().unwrap();

        let w_nums = Game::parse_game_numbers(winning);
        let p_nums = Game::parse_game_numbers(picked);

        (w_nums, p_nums)
    }

    fn parse_game_numbers(num_str: &str) -> Vec<i32> {
        num_str
            .split_whitespace()
            .map(|n| n.parse::<i32>().expect("Failed to parse to i32"))
            .collect::<Vec<i32>>()
    }
}

