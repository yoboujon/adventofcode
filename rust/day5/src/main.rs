use std::fs::read_to_string;

fn analyse(str: &String) -> (u32, Vec<(u32, u32, u32)>, Vec<Vec<&str>>) {
    // Initializing some values.
    let mut info: Vec<_> = str.split("\r\n").collect();
    let mut crates: &[&str] = &[""];
    let mut cmd: &[&str] = &[""];
    let mut crates_str: Vec<Vec<&str>> = Vec::new();
    let mut commands_tuple: Vec<(u32, u32, u32)> = Vec::new();

    // Spliting the str into a cmd part, and crates part.
    for (i, txt) in info.iter().enumerate() {
        if txt.len() == 0 {
            info.remove(i);
            (crates, cmd) = info.split_at(i);
            break;
        }
    }
    // Gathering the horizontal number of crates
    let max_crates: Vec<u32> = crates
        .last()
        .unwrap()
        .to_string()
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();

    // Gathering each crate Letters
    for (i, str) in crates.iter().enumerate() {
        if i == crates.len()-1 {
            break;
        }
        crates_str.push(str.split_whitespace().collect());
    }

    // Rearanging commands as simplier datatypes to process.
    for str in cmd {
        let str_temp : Vec<&str> = str.split_whitespace().collect();
        let tuple_temp : Vec<Option<u32>> = str_temp.iter().map_while(|str| Some(str.parse().ok())).collect();
        commands_tuple.push((tuple_temp[1].unwrap(), tuple_temp[3].unwrap(), tuple_temp[5].unwrap()));
    }

    // Returning every info as a tuple
    (
        max_crates.iter().max().unwrap().to_owned(),
        commands_tuple.to_owned(),
        crates_str.to_owned()
    )
}
fn main() {
    let file_content = read_to_string("exemple").unwrap();
    let (max, cmd, crates) = analyse(&file_content);
    println!("max = {}\ncmd = {:?}\ncrates = {:?}", max, cmd, crates);
}
