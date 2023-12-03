use std::fs::read_to_string;

fn part1(str: &String) -> Vec<u32> {
    let mut num_vec = Vec::new(); 
    let line : Vec<&str> = str.split('\n').collect();
    for l in line {
        let num : Vec<u8> = l.as_bytes().iter().filter(|ch| ch.is_ascii_digit()).cloned().collect();
        let mut str_num = String::new();
        if num.len() >= 2 {
            str_num.push(num.first().unwrap().clone() as char);
            str_num.push(num.last().unwrap().clone() as char);
        } else {
            str_num.push(num.first().unwrap().clone() as char);
            str_num.push(num.first().unwrap().clone() as char);
        }
        num_vec.push(str_num);
    }
    println!("{:?}", num_vec);
    num_vec.iter().map(|str| str.parse().unwrap()).collect()
}

fn main() {
    let str = read_to_string("input").unwrap();
    let part1_result : u32 = part1(&str).iter().sum();
    println!("total part1: {}", part1_result);
}
