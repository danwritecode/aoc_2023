use std::collections::HashMap;

pub fn part_one() -> i32 {
    let input = include_str!("../input");
    let coords = input
        .lines()
        .map(|l| {
            let num_chars = l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
            let first = num_chars.first().unwrap();
            let last = num_chars.last().unwrap();
            let combined = format!("{}{}", first, last);

            combined.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();

    coords.into_iter().sum::<i32>()
}

pub fn part_two() -> i32 {
    let input = include_str!("../input");
    let word_map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let coords = input
        .lines()
        .map(|l| {
            let mut char_map = vec![];

            l.chars().enumerate().for_each(|(i, c)| { 
                if c.is_numeric() {
                    char_map.push((i, c));
                }
            });

            word_map.iter().for_each(|(w, n)| {
                let m_idxs = l.match_indices(w).map(|(i, _w)| i).collect::<Vec<usize>>();
                m_idxs.iter().for_each(|i| {
                    char_map.push((*i, *n));
                });
            });

            char_map.sort_by(|a, b| {
                a.0.cmp(&b.0)
            });

            let chars = char_map.iter_mut()
                .map(|c| c.1)
                .collect::<Vec<char>>();

            let first = chars.first().unwrap();
            let last = chars.last().unwrap();
            let combined = format!("{}{}", first, last);

            combined.parse::<i32>().unwrap()

        })
        .collect::<Vec<i32>>();

    coords.into_iter().sum::<i32>()
}
