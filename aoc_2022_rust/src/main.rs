use std::fs;
mod day_01;
fn main() {
    let input = fs::read_to_string("./src/day_01_input.txt").expect("Error reading file");
    let day_1_result = day_01::day_01_a(&input);    
    println!("Day 1 result: {}", day_1_result);   
    let day_1_b_result = day_01::day_01_b(&input);    
    println!("Day 1b result: {}", day_1_b_result);
}
