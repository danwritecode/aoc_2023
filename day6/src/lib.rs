pub fn part_one() -> i64 {
    let input = include_str!("../input");
    let lines = input.lines().collect::<Vec<&str>>();
    let groups = process_input_pt_one(lines);

    groups
        .into_iter()
        .map(|(t, d)| calc_winning_hold_times(t, d))
        .product::<i64>()
}

pub fn part_two() -> i64 {
    let input = include_str!("../input");
    let lines = input.lines().collect::<Vec<&str>>();
    let (time, distance) = process_input_pt_two(lines);

    calc_winning_hold_times(time, distance)
}


type CtWinningHoldTimes = i64;
fn calc_winning_hold_times(total_ms: i64, target_dist_mm: i64) -> CtWinningHoldTimes { 
    let mut winning_ct = 0;

    for hold_ms in 1..total_ms + 1 {
        let allotted_travel_time = total_ms - hold_ms;
        let calcd_distance = allotted_travel_time * hold_ms;

        if calcd_distance > target_dist_mm {
            winning_ct += 1;
        }
    }

    winning_ct
}

fn process_input_pt_one(lines: Vec<&str>) -> Vec<(i64, i64)> {
    let times = parse_line_pt_one(lines[0]);
    let distances = parse_line_pt_one(lines[1]);

    let grouped = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(a, b)| (a, b))
        .collect::<Vec<(i64, i64)>>();

    grouped
}

fn parse_line_pt_one(line: &str) -> Vec<i64> {
    line
        .split(":")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()

}

fn process_input_pt_two(lines: Vec<&str>) -> (i64, i64) {
    let time = parse_line_pt_two(lines[0]);
    let distance = parse_line_pt_two(lines[1]);

    (time, distance)
}

fn parse_line_pt_two(line: &str) -> i64 {
    line
        .split(":")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .trim()
        .chars()
        .fold("".to_string(), |mut acc, c| {
            if c.is_numeric() {
                acc.push(c);
            }
            acc
        })
        .parse::<i64>()
        .expect("Failed to parse to i64")

}
