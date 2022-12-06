type WorkRange = (u32, u32);

fn create_tuple(input: &str) -> WorkRange {
    let parts = input.split('-').collect::<Vec<&str>>();
    (
        parts[0].parse::<u32>().unwrap(),
        parts[1].parse::<u32>().unwrap(),
    )
}

fn build_pairs(input: &str) -> (WorkRange, WorkRange) {
    let result: Vec<WorkRange> = input.split(',').map(create_tuple).collect();
    (result[0], result[1])
}
fn does_it_contain(input: (WorkRange, WorkRange)) -> u32 {
    let left = input.0;
    let right = input.1;
    // if left contains right
    if left.0 <= right.0 && left.1 >= right.1 {
        return 1;
    }
    // if right contains left
    if right.0 <= left.0 && right.1 >= left.1 {
        return 1;
    }
    0
}

fn does_it_overlap(input: (WorkRange, WorkRange)) -> u32 {
    let left = input.0;
    let right = input.1;
    // is there any overlap
    if left.1 >= right.0 && right.1 >= left.0 {
        return 1;
    }
    // is there any overlap
    if right.1 >= left.0 && left.1 >= right.0 {
        return 1;
    }
    0
}

pub fn day_04_a(input: &str) -> u32 {
    input.lines().map(build_pairs).map(does_it_contain).sum()
}
pub fn day_04_b(input: &str) -> u32 {
    input.lines().map(build_pairs).map(does_it_overlap).sum()
}
