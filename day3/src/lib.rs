
pub fn part_one() -> i32 {
    let input = include_str!("../input");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut part_num_coords: Vec<PartNumber> = vec![];
    let mut symbol_coords: Vec<(usize, usize)> = vec![];

    map_coords(lines, &mut part_num_coords, &mut symbol_coords);

    let part_num_coords = part_num_coords
        .into_iter()
        .filter_map(|p| {
            println!("current part number: {} | position: {:?}", p.number, p.position);
            // check above +-1
            // check below +-1
            // check same line one left
            // check same line one right

            let above_idx = p.position.1.checked_sub(1); 
            let below_idx = p.position.1 + 1; 
            let left_idx = p.position.0.checked_sub(1).map_or(0, |i| i); 
            let right_idx = p.position.0 + p.number.to_string().len(); 

            let mut coords_to_check: Vec<(usize, usize)> = vec![];

            for l in 0..p.number.to_string().len() + 2 {
                if let Some(a) = above_idx {
                    println!("pushing above: {:?}", (left_idx + l, a));
                    coords_to_check.push((left_idx + l, a))
                }

                println!("pushing below: {:?}", (left_idx + l, below_idx));
                coords_to_check.push((left_idx + l, below_idx));
                println!("pushing right: {:?}", (right_idx, p.position.1));
                coords_to_check.push((right_idx, p.position.1));
                println!("pushing left: {:?}", (left_idx, p.position.1));
                coords_to_check.push((left_idx, p.position.1));
            }
    
            for c in &coords_to_check {
                if symbol_coords.contains(c) {
                    return Some(p.number);
                }
            }
            None
        })
        .sum::<i32>();

    println!("{:?}", part_num_coords);

    todo!()
}

fn map_coords(
    lines: Vec<&str>,
    part_num_coords: &mut Vec<PartNumber>, 
    symbol_coords: &mut Vec<(usize, usize)>
) {
    lines
        .iter()
        .enumerate()
        .for_each(|(l_idx, l)| {
            let symbols = l.chars().enumerate().filter_map(|(c_idx, c)| {
                if c.is_ascii_punctuation() && c != '.' {
                    return Some((c_idx, l_idx));
                }
                None
            }).collect::<Vec<(usize, usize)>>();

            symbol_coords.extend(symbols);

            let mut part_nums = vec![];
            l.chars().enumerate().fold("".to_string(), |mut acc, (c_idx, c)| {
                if c.is_numeric() {
                    acc.push(c);
                    return acc;
                }

                // if not numeric
                if acc != "".to_string() {
                    let coord = (c_idx - acc.len(), l_idx);
                    let number = acc.parse::<i32>().expect("Failed to parse acc num");
                    part_nums.push(PartNumber::new(number, coord));
                    acc = "".to_string();
                }

                acc
            });

            part_num_coords.extend(part_nums);
        });

}

#[derive(Debug)]
struct PartNumber {
    number: i32,
    position: (usize, usize),
}

impl PartNumber {
    fn new(number: i32, position: (usize, usize)) -> Self {
        PartNumber { number, position }
    }
}
