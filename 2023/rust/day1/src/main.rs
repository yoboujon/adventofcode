use std::fs::read_to_string;

const DIGITS : [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn contains_digit(txt: &str, actual_index: usize) -> Option<usize> {
    let mut digit_index: Option<usize> = Option::None;

    for i in [3, 4, 5] {
        let max_str = if actual_index + i >= txt.len() {
            txt.len()
        } else {
            actual_index + i
        };
        digit_index = DIGITS
        .iter()
        .position(|&digit| txt[actual_index..max_str].eq(digit));
        if digit_index.is_some() {
            break;
        }
    }
    digit_index
}

fn convert_spelled_digit(txt: &str) -> String {
    // static variable
    let digits_char: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    // mutable variables
    let mut finished = false;
    let mut return_string = txt.to_string();

    while !finished {
        let mut temp_string = return_string.clone();
        for (str_index, _) in return_string.as_bytes().iter().enumerate() {
            if str_index == return_string.len() - 1 {
                finished = true;
            }
            if let Some(i) = contains_digit(&return_string,str_index) {
                // Transforming the string
                let (first, last) = return_string.split_at(str_index);
                let (_, last) = last.split_at(1);
                temp_string = first.to_string() + digits_char[i].to_string().as_str() + last;
                break;
            }
        }
        return_string = temp_string;
    }
    return_string
}

fn part1(str: &String) -> Vec<u64> {
    let mut num_vec = Vec::new();
    let line: Vec<&str> = str.split('\n').collect();
    for l in line {
        num_vec.push(algorithm(l.to_string()));
    }
    println!("{:?}", num_vec);
    num_vec.iter().map(|str| str.parse().unwrap()).collect()
}

fn part2(str: &String) -> Vec<u64> {
    let mut num_vec = Vec::new();
    let line: Vec<&str> = str.split('\n').collect();
    for l in line {
        num_vec.push(algorithm(convert_spelled_digit(l)));
    }
    for num in &num_vec {
        println!("{}", num);
    }
    num_vec.iter().map(|str| str.parse().unwrap()).collect()
}

fn algorithm(str: String) -> String {
    let num: Vec<u8> = str
        .as_bytes()
        .iter()
        .filter(|ch| ch.is_ascii_digit())
        .cloned()
        .collect();
    let str_num: String;
    if num.len() == 0 {
        str_num = "00".to_string();
    } else if num.len() >= 2 {
        str_num = (num.first().unwrap().clone() as char).to_string() + (num.last().unwrap().clone() as char).to_string().as_str();
    } else {
        str_num = (num.first().unwrap().clone() as char).to_string() + (num.first().unwrap().clone() as char).to_string().as_str();
    }
    str_num
}

fn main() {
    let str = read_to_string("input").unwrap();
    let part1_result: u64 = part1(&str).iter().sum();
    println!("part1 result: {}", part1_result);
    let part2_result: u64 = part2(&str).iter().sum();
    println!("part2 result: {}", part2_result);
}
