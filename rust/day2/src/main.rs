use std::fs::read_to_string;
use std::collections::HashMap;
mod rps;

fn main()
{
    // Checking if the file is found.
    let mut oponent : Vec<String> = read_to_string("input").unwrap().split_whitespace().map(|s| s.to_string()).collect();
    let mut player = oponent.clone();
    oponent.retain(|s| s == "A" || s == "B" || s == "C");
    player.retain(|s| s == "X" || s == "Y" || s == "Z");
    assert!(oponent.len() == player.len(), "Oponent and Player don't play as many turns.");

    let converter : HashMap<char, rps::RockPaperScissors> = rps::get_converter();
    let player = converter.get(&'Y').unwrap().battle(converter.get(&'A').unwrap());
    println!("First battle = {}", player);
    let player = converter.get(&'X').unwrap().battle(converter.get(&'B').unwrap());
    println!("Second battle = {}", player);
    let player = converter.get(&'Z').unwrap().battle(converter.get(&'C').unwrap());
    println!("Third battle = {}", player);
}