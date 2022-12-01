use std::fs;
mod day_01;

fn get_input_day(number: u8) -> String {
    fs::read_to_string(format!("./src/day_{:02}_input.txt", number)).expect("Error reading file")
}


fn run_fn_day<R>(number: u8, func: fn(&str) -> R) -> R {
    func(&get_input_day(number))
}

fn main() {
    let day_1_result = run_fn_day(1, day_01::day_01_a);    
    println!("Day 1 result: {}", day_1_result);   
    let day_1_b_result = run_fn_day(1, day_01::day_01_b);   
    println!("Day 1b result: {}", day_1_b_result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_day_01_a() {
        assert_eq!(run_fn_day(1, day_01::day_01_a), 72478);
    }

    #[test]
    fn test_day_01_b() {
        assert_eq!(run_fn_day(1, day_01::day_01_b), 210367);
    }
}