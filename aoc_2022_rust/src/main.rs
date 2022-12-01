use std::fs;
mod day_01;

fn main() {
    let input = fs::read_to_string("./src/day_01/input.txt").expect("Error reading file");
    let day_1_result = crate::day_01::dwarf_calories::day_01(&input);    
    println!("Day 1 result: {}", day_1_result);   
    let day_1_b_result = crate::day_01::dwarf_calories::day_01_b(&input);    
    println!("Day 1b result: {}", day_1_b_result);   
}
