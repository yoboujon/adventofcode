use std::fs::read_to_string;

fn convert(num: u8) -> u8 {
    if num >= ('a' as u8) {
        num + 1 - ('a' as u8)
    } else {
        num +27 - ('A' as u8)
    }
}

fn main() {
    let binding = read_to_string("input").unwrap();
    let file_content: Vec<&str> = binding.split("\r\n").collect();
    let mut ruckstack: Vec<u64> = Vec::new();
    for con in &file_content {
        let splitted = con.split_at(con.len() / 2);
        let mut iter = splitted
            .0
            .chars()
            .into_iter()
            .filter(|ch| splitted.1.find(*ch).is_some());
        ruckstack.push(convert(iter.next().unwrap() as u8) as u64);
    }
    println!("Sum of the priorities : {}", ruckstack.iter().sum::<u64>());
}
