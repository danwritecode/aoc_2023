struct CubeColorsLimits {
    red: i32,
    blue: i32,
    green: i32,
}

impl CubeColorsLimits {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        CubeColorsLimits { red, blue, green }
    }
}

#[derive(Debug)]
struct CubeSet {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>
}

impl Default for CubeSet {
    fn default() -> Self {
        CubeSet {
            red: None,
            green: None,
            blue: None
        }
    }
}

pub fn part_one() -> i32 {
    let input = include_str!("../input");
    let lines = input.lines().collect::<Vec<&str>>();

    let limits = CubeColorsLimits::new(12, 13, 14);
    let mut game_sum = 0;

    lines
        .iter()
        .for_each(|l| {
            let game = parse_game(l);
            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;

            for g in &game.1 {
                if let Some(ct) = g.red {
                    if ct > red_max {
                        red_max = ct;
                    }
                }
                if let Some(ct) = g.green {
                    if ct > green_max {
                        green_max = ct;
                    }
                }
                if let Some(ct) = g.blue {
                    if ct > blue_max {
                        blue_max = ct;
                    }
                }
            }

            if red_max <= limits.red && green_max <= limits.green && blue_max <= limits.blue {
                game_sum += game.0
            }
        });

    game_sum
}

pub fn part_two() -> i32 {
    let input = include_str!("../input");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut power_sum = 0;

    lines
        .iter()
        .for_each(|l| {
            let game = parse_game(l);
            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;

            for g in &game.1 {
                if let Some(ct) = g.red {
                    if ct > red_max {
                        red_max = ct;
                    }
                }
                if let Some(ct) = g.green {
                    if ct > green_max {
                        green_max = ct;
                    }
                }
                if let Some(ct) = g.blue {
                    if ct > blue_max {
                        blue_max = ct;
                    }
                }
            }

            let power = red_max * green_max * blue_max;
            power_sum += power;
        });

    power_sum
}

fn parse_game_id(label: &str) -> i32 {
    let ws_index = label.find(" ").unwrap() + 1;
    let id = &label[ws_index..];
    id.parse::<i32>().unwrap()
}

fn parse_cube(cube: &str) -> (i32, String) {
    let ws_index = cube.find(" ").unwrap();
    let count = &cube[..ws_index];
    let color = &cube[ws_index + 1..];
    (count.parse::<i32>().unwrap(), color.to_string())
}

fn parse_game(line: &str) -> (i32, Vec<CubeSet>) {
    let game_split = line.split(":").collect::<Vec<&str>>();
    let game_id = parse_game_id(game_split[0]);
    let subsets = game_split[1].split(";").map(|s| s.trim()).collect::<Vec<&str>>();

    let subsets = subsets
        .iter()
        .map(|g| {
            let cube_sets = g.split(",").map(|c| c.trim()).collect::<Vec<&str>>();
            let cube_sets = cube_sets.iter().fold(CubeSet::default(), |mut acc, c| {
                let (ct, color) = parse_cube(c);

                match color.as_str() {
                    "red" => {
                        acc.red = Some(ct);
                    },
                    "green" => {
                        acc.green = Some(ct);
                    },
                    "blue" => {
                        acc.blue = Some(ct);
                    },
                    _ => unreachable!()
                }

                acc
            });

            cube_sets
        })
        .collect::<Vec<CubeSet>>();

    (game_id, subsets)
}
