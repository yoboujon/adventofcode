use std::fs::read_to_string;

fn convert(vec: &Vec<&str>, index: usize, str_index: usize) -> u32 {
    let two_num: Vec<&str> = vec[index].split("-").collect();
    two_num[str_index].parse().unwrap()
}

fn first_part(str: &String) {
    let mut count = 0;
    let pairs: Vec<Vec<&str>> = str
        .split("\r\n")
        .map(|ch| ch.split(",").collect())
        .collect();
    for pair in &pairs {
        if convert(pair, 0, 0) <= convert(pair, 1, 0) && convert(pair, 0, 1) >= convert(pair, 1, 1)
            || convert(pair, 0, 0) >= convert(pair, 1, 0)
                && convert(pair, 0, 1) <= convert(pair, 1, 1)
        {
            count += 1;
        }
    }
    println!("Final Count: {}", count);
}

fn second_part(str: &String) {
    let mut count = 0;
    let pairs: Vec<Vec<&str>> = str
        .split("\r\n")
        .map(|ch| ch.split(",").collect())
        .collect();
    for pair in &pairs {
        if convert(pair, 0, 1) >= convert(pair, 1, 0) && convert(pair, 0, 0) <= convert(pair, 1, 1)
        {
            count += 1
        }
    }
    println!("Part 2 Count : {}", count);
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    first_part(&file_content);
    second_part(&file_content);
}
