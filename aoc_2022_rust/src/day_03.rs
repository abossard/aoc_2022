use array_tool::vec::Intersect;

fn convert_to_prio(c: char) -> u32 {
    let ascii = c as u32;
    let value = match ascii {
        97..=122 => ascii - 96,
        _ => ascii - 38,
    };
    value
}

fn parse_rucksack(input: &str) -> (Vec<u32>, Vec<u32>) {
    let priorities = input.chars().map(convert_to_prio).collect::<Vec<u32>>();
    let left_compartment = &priorities[..priorities.len() / 2];
    let right_compartment = &priorities[priorities.len() / 2..];
    (left_compartment.to_owned(), right_compartment.to_owned())
}

fn find_common_priorities(left: Vec<u32>, right: Vec<u32>) -> u32 {
    left.intersect(right).into_iter().sum()
}

fn merge_items_by_three(input: Vec<&str>) -> Vec<Vec<&str>> {
    let mut result = Vec::new();
    let mut temp = Vec::new();
    for item in input {
        temp.push(item);
        if temp.len() == 3 {
            result.push(temp);
            temp = Vec::new();
        }
    }
    result
}

fn find_common_letter(input: Vec<&str>) -> char {
    for letter in input[0].chars() {
        let mut found = true;
        for item in &input[1..] {
            if !item.contains(letter) {
                found = false;
                break;
            }
        }
        if found {
            return letter;
        }
    }
    ' '
}

pub fn day_03_a(input: &str) -> u32 {
    input
        .lines()
        .map(parse_rucksack)
        .map(|r| find_common_priorities(r.0, r.1))
        .sum()
}

pub fn day_03_b(input: &str) -> u32 {
    merge_items_by_three(input.lines().collect())
        .into_iter()
        .map(find_common_letter)
        .map(convert_to_prio)
        .sum()
}
