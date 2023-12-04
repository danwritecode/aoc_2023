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
    let mut lines = input.lines().collect::<Vec<&str>>();

    for i in 0..lines.len() {
        let game = Game::new(lines[i]);
        let matches = game.calc_matches();
        let match_slice = lines[i + 1..i + matches + 1].to_vec();

        println!("game {} has {} matches | slice: {:?}", game.id, matches, match_slice);

        for (mli, ml) in match_slice.iter().enumerate() {
            lines.insert(mli + i, ml);
        }

        println!("new lines: {:?}", lines);
    }

    // println!("line len: {}", lines.len());

    todo!()
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
        let matching = &self.picked
            .iter()
            .map(|p| {
                if self.winning.contains(p) {
                    return 1;
                }
                return 0;
            })
            .sum::<u32>();

        if *matching == 0 as u32 {
            return 0;
        }

        let base: i32 = 2;
        let total = base.pow(matching - 1);
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

