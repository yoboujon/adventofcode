use std::fs::read_to_string;
use std::collections::HashMap;
mod rps;

fn main()
{
    // Checking if the file is found.
    let mut score : u64 = 0;
    let mut oponent : Vec<String> = read_to_string("input").unwrap().split_whitespace().map(|s| s.to_string()).collect();
    let mut player = oponent.clone();
    oponent.retain(|s| s == "A" || s == "B" || s == "C");
    player.retain(|s| s == "X" || s == "Y" || s == "Z");
    assert!(oponent.len() == player.len(), "Oponent and Player don't play as many turns.");

    let converter : HashMap<char, rps::RockPaperScissors> = rps::get_converter();
    for tools in oponent.iter().zip(player.iter_mut()) {
        let o = tools.0.char_indices().next().unwrap().1;
        let p = tools.1.char_indices().next().unwrap().1;
        score += converter.get(&p).unwrap().battle(converter.get(&o).unwrap());
    }
    println!("Final score: {}", score);
}