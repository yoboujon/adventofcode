use std::fs::read_to_string;

fn analyse(str: &String) -> (u32, Vec<(u32, u32, u32)>, Vec<Vec<char>>) {
    // Initializing some values.
    let mut info: Vec<_> = str.split("\r\n").collect();
    let mut crates_str: Vec<Vec<char>> = Vec::new();
    let mut commands_tuple: Vec<(u32, u32, u32)> = Vec::new();

    // Spliting the str into a cmd part, and crates part.
    let split_index = info
        .iter()
        .enumerate()
        .find(|txt| txt.1.len() == 0)
        .unwrap()
        .to_owned()
        .0;
    info.remove(split_index);
    let (crates, cmd) = info.split_at(split_index);

    // Gathering the horizontal number of crates
    let max_crates: Vec<u32> = crates
        .last()
        .unwrap()
        .to_string()
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();
    let max = max_crates.iter().max().unwrap().to_owned();

    // Gathering each crate Letters
    for _i in 0..max {
        crates_str.push(Vec::new());
    }
    for (i, str) in crates.iter().rev().enumerate() {
        if i == 0 {
            continue;
        }
        for y in 0..max {
            let ch = str.as_bytes()[((y * 4) + 1) as usize];
            if ch != (' ' as u8) {
                crates_str[y as usize].push(ch as char);
            }
        }
    }

    // Rearanging commands as simplier datatypes to process.
    for str in cmd {
        let tuple_temp: Vec<Option<u32>> = str
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map_while(|str| Some(str.parse().ok()))
            .collect();
        commands_tuple.push((
            tuple_temp[1].unwrap(),
            tuple_temp[3].unwrap(),
            tuple_temp[5].unwrap(),
        ));
    }

    // Returning every info as a tuple
    (
        max,
        commands_tuple.to_owned(),
        crates_str.to_owned(),
    )
}

fn first_part(cmd: &Vec<(u32, u32, u32)>, crates: &Vec<Vec<char>>) -> String
{
    // Algorithm
    let mut boxes = crates.clone();
    for operation in cmd {
        let mut moving_crates: Vec<char> = Vec::new();
        for _ in 0..(operation.0) {
            moving_crates.push(boxes[(operation.1-1) as usize].last().unwrap().to_owned());
            boxes[(operation.1-1) as usize].pop();
        }
        for mv in moving_crates {
            boxes[(operation.2-1) as usize].push(mv);
        }
    }
    
    // Returning chars
    let mut return_chars: Vec<char> = Vec::new();
    for b in boxes {
        return_chars.push(b.last().unwrap().to_owned());
    }
    return_chars.into_iter().collect()
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    let (max, cmd, crates) = analyse(&file_content);
    println!("max = {}\ncmd = {:?}\ncrates = {:?}", max, cmd, crates);
    println!("First part: {:?}", first_part(&cmd, &crates));
}
