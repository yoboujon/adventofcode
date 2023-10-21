use std::collections::HashMap;
use std::fs::read_to_string;

fn is_duplicate(str: &String) -> Option<char> {
    let mut char_hash: HashMap<char, u8> = HashMap::new();
    for ch in str.chars() {
        char_hash.entry(ch).and_modify(|x| *x = 2).or_insert(1);
    }
    str.chars().into_iter().find(|ch| char_hash[ch] == 2)
}

fn part_one(str: String) {
    let (temp_str, str_iter) = str.split_at(4);
    let mut count_str = temp_str.to_string();
    let mut count = 4;
    for ch in str_iter.chars().into_iter() {
        match is_duplicate(&count_str) {
            None => {
                break;
            }
            Some(_) => {
                count_str.remove(0);
                count_str.push(ch);
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    part_one(file_content);
}
