use std::fs;
mod day_01;
mod day_02;
mod day_03;
mod statics;

fn get_input_day(number: u8) -> String {
    fs::read_to_string(format!("./src/day_{:02}_input.txt", number)).expect("Error reading file")
}

fn run_fn_day<R>(number: u8, func: fn(&str) -> R) -> R {
    func(&get_input_day(number))
}

fn main() {
    run_fn_day(3, day_03::day_03_a);
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

    #[test]
    fn test_day_02_a() {
        assert_eq!(run_fn_day(2, day_02::day_02_a), 15572);
    }

    #[test]
    fn test_day_02_b() {
        assert_eq!(run_fn_day(2, day_02::day_02_b), 16098);
    }

    #[test]
    fn test_day_03_a() {
        assert_eq!(run_fn_day(3, day_03::day_03_a), 7785);
    }

    #[test]
    fn test_day_03_b() {
        assert_eq!(run_fn_day(3, day_03::day_03_b), 70);
    }
}
