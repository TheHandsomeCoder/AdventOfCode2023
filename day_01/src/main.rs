
fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let sum: u32 = input.lines().map(|l| extract_calibration_value(l)).sum();            
    println!("Part 1 Solution {}", sum);
}

fn part_two(input: &str) {
    let sum: u32 = input.lines()
        .map(|l| replace_spelled_with_number(l))
        .map(|l| extract_calibration_value(&l))
        .sum();
    println!("Part 2 Solution {}", sum);
}

fn extract_calibration_value(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    return 10 * digits.first().unwrap() + digits.last().unwrap();
}

fn replace_spelled_with_number(line: &str) -> String {
    return line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
}