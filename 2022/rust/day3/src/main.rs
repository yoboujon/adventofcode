use std::fs::read_to_string;

fn convert(num: u8) -> u8 {
    if num >= ('a' as u8) {
        num + 1 - ('a' as u8)
    } else {
        num +27 - ('A' as u8)
    }
}

fn first_part(str: &String) {
    let file_content: Vec<&str> = str.split("\r\n").collect();
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

fn second_part(str: &String) {
    let file_content: Vec<&str> = str.split("\r\n").collect();
    let mut ruckstack: Vec<u64> = Vec::new();
    let mut badge: Vec<[&str; 3]> = Vec::new();
    let mut j : i64 = -1;

    for (i, con) in file_content.iter().enumerate() {
        if i%3 == 0 {
            badge.push(["","",""]);
            j +=1;
        }
        badge[j as usize][i%3] = con; 
    }
    for strs in &badge {
        //let badge_second = strs[1].to_owned()+&strs[2].to_owned();
        let iter : Vec<char> = strs[0]
            .chars()
            .into_iter()
            .filter(|ch| strs[1].find(*ch).is_some())
            .filter(|ch| strs[2].find(*ch).is_some())
            .collect();
        ruckstack.push(convert(iter[0] as u8) as u64);
    }
    println!("Sum of the badges : {}", ruckstack.iter().sum::<u64>());
}

fn main() {
    let binding = read_to_string("input").unwrap();
    first_part(&binding);
    second_part(&binding);
}
