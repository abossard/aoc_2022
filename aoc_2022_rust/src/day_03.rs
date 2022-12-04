use array_tool::vec::Intersect;
use std::collections::HashMap;

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

fn count_letters(input: &str) -> HashMap<char, u32> {
    // create hashmap with letter as key and count as value
    let mut map = std::collections::HashMap::new();
    for c in input.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    map
}

fn find_common_letter(input: Vec<HashMap<char, u32>>) -> char {
    // find the letter that is in all three maps
    let mut result = ' ';
    for (key, value) in input[0].iter() {
        if input[1].contains_key(key) && input[2].contains_key(key) {
            if *value > 1 {
                result = *key;
            }
        }
    }
    println!("{:?}", input);
    '1'
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
        .map(|r| find_common_letter(r.into_iter().map(count_letters).collect()))
        .map(convert_to_prio)
        .sum()
}
