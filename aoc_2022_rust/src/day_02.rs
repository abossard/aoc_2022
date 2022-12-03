use phf::phf_map;

static HANDS: phf::Map<&'static str, u8> = phf_map! {
    "A" => 1, // rock
    "X" => 1, // rock
    "B" => 2, // paper
    "Y" => 2, // paper
    "C" => 3, // scissors
    "Z" => 3, // scissors
};

fn win_points(left: u8, right: u8) -> (u8, u8) {
    let points = match (left, right) {
        (1, 2) => (0, 6),
        (1, 3) => (6, 0),
        (2, 1) => (6, 0),
        (2, 3) => (0, 6),
        (3, 1) => (0, 6),
        (3, 2) => (6, 0),
        _ => (3, 3),
    };
    (left + points.0, right + points.1)
}
fn win_points_2(left: u8, right: u8) -> (u8, u8) {
    let new_right = match right {
        1 => match left {
            // loose
            1 => 3,
            2 => 1,
            _ => 2,
        },
        3 => match left {
            // win
            1 => 2,
            2 => 3,
            _ => 1,
        },
        _ => left, // draw
    };
    win_points(left, new_right)
}
fn parse_line(line: &str, func: fn(u8, u8) -> (u8, u8)) -> (u8, u8) {
    let hands: Vec<&str> = line.split_whitespace().collect();
    let left = HANDS[hands[0]];
    let right = HANDS[hands[1]];
    func(left, right)
}

fn make_my_day(input: &str, func: fn(u8, u8) -> (u8, u8)) -> u32 {
    input
        .lines()
        .map(|line| parse_line(line, func).1 as u32)
        .sum()
}

pub fn day_02_a(input: &str) -> u32 {
    make_my_day(input, win_points)
}

pub fn day_02_b(input: &str) -> u32 {
    make_my_day(input, win_points_2)
}
