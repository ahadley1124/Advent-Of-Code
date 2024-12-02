use std::io::BufRead;

mod part_two;

fn open_input_file() -> std::io::BufReader<std::fs::File> {
    let file = std::fs::File::open("input.txt").expect("Cannot open file");
    std::io::BufReader::new(file)
}

fn read_input() -> Vec<String> {
    let reader = open_input_file();
    reader.lines().map(|l| l.expect("Cannot read line")).collect()
}

fn seperate_input(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut first = Vec::new();
    let mut second = Vec::new();
    // Split the input into two parts, the first number is the first numbers in the line and the second number is the lines after the spaces in the line
    for line in input {
        let parts: Vec<&str> = line.split("   ").collect();
        first.push(parts[0].to_string());
        second.push(parts[1].to_string());
    }
    (first, second)
}

fn sort_by_value(input: Vec<String>) -> Vec<String> {
    let mut numbers: Vec<i32> = input.iter().map(|s| s.parse().unwrap()).collect();
    numbers.sort();
    numbers.iter().map(|n| n.to_string()).collect()
}

fn convert_to_int(input: Vec<String>) -> Vec<i32> {
    input.iter().map(|s| s.parse().unwrap()).collect()
}

fn subtract_values(string1: String, string2: String) -> String {
    let number1: i32 = string1.parse().unwrap();
    let number2: i32 = string2.parse().unwrap();
    if number1 > number2 {
        (number1 - number2).to_string()
    } else {
        (number2 - number1).to_string()
    }
}

fn add_all_values(input: Vec<String>) -> i32 {
    let numbers = convert_to_int(input);
    numbers.iter().sum()
}

fn main() {
    let input = read_input();
    let (mut first, mut second) = seperate_input(input);
    let first_mod = convert_to_int(sort_by_value(first));
    let second_mod = convert_to_int(sort_by_value(second));
    let mut result = Vec::new();
    for i in 0..first_mod.len() {
        result.push(subtract_values(first_mod[i].to_string(), second_mod[i].to_string()));
    }
    println!("Result: {}", add_all_values(result));
    println!("\n\n\n\n\n");
    println!("Part Two: {}", part_two::run_similarity());
}
