use std::io::BufRead;

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


fn total_number_recursion(column1: Vec<i32>, column2: Vec<i32>) -> Vec<(i32, i32)> {
    let mut results: Vec<(i32, i32)> = Vec::new();
    for i in 0..column1.len() {
        for j in 0..column2.len() {
            // take i and count how many times it equal to j and add it to the results like this: (i, number of times it equal to j)
            results.push((column1[i], column2.iter().filter(|&n| *n == column1[i]).count().try_into().unwrap()));
        }
    }
    results
}

fn multiply_for_similarity(recursion: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut results: Vec<(i32, i32)> = Vec::new();
    // Multiply the first number with the second number in the tuple
    for i in 0..recursion.len() {
        results.push((recursion[i].0, recursion[i].0 * recursion[i].1));
    }
    results
}

fn multiply_for_final_result(column1: Vec<i32>, similarity_score: Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    for i in 0..column1.len() {
        // find i in the first number of the tuple and add the second number of the tuple to the result, if it is not found add 0
        result += similarity_score.iter().find(|&n| n.0 == column1[i]).unwrap_or(&(0, 0)).1;
    }
    result
}

pub fn run_similarity()-> i32 {
    let input = read_input();
    let (mut first, second) = seperate_input(input);
    let first_mod_mod = convert_to_int(sort_by_value(first));
    let second_mod_mod = convert_to_int(sort_by_value(second));
    let similarity_score = multiply_for_similarity(total_number_recursion(first_mod_mod.clone(), second_mod_mod));
    let result = multiply_for_final_result(first_mod_mod, similarity_score);
    println!("Result: {}", result);
    result
}