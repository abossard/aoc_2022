/// sums up a string of lines with some empty lines in between
/// 1. for each line
/// 2. parse to u32 and force unwrap (not nice, but good enough)
/// 3. sum up
fn dwarf_sum(input: &str)-> u32 {
    input.lines().map(|line| line.parse::<u32>().unwrap()).sum()
}

/// splits the input on empty lines (double newlines) and then calls dwarf_sum on each part
fn dwarf_creater(input: &str) -> Vec<u32> {
    let mut dwarfs:Vec<u32> = input.split("\n\n").map(dwarf_sum).collect();
    dwarfs.sort();
    return dwarfs;
}
    
pub fn day_01_a(input: &str) -> u32 {
    let dwarfs = dwarf_creater(input);
    return dwarfs[dwarfs.len() - 1];
}

pub fn day_01_b(input: &str) -> u32 {
    let dwarfs = dwarf_creater(input);
    return dwarfs.as_slice()[dwarfs.len() -3..].iter().sum();
}
