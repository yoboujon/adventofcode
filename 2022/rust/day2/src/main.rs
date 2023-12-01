use std::collections::HashMap;
use std::fs::read_to_string;
mod rps;

fn first_strategy(player: &mut Vec<String>, oponent: &Vec<String>) {
    let mut score: u64 = 0;
    let converter: HashMap<char, rps::RockPaperScissors> = rps::get_converter();
    for tools in oponent.iter().zip(player.iter_mut()) {
        let o = tools.0.char_indices().next().unwrap().1;
        let p = tools.1.char_indices().next().unwrap().1;
        score += converter
            .get(&p)
            .unwrap()
            .battle(converter.get(&o).unwrap());
    }
    println!("Final score (first method): {}", score);
}

fn second_strategy(player: &mut Vec<String>, oponent: &Vec<String>) {
    let mut score: u64 = 0;
    let converter: HashMap<char, rps::RockPaperScissors> = rps::get_converter();
    for tools in oponent.iter().zip(player.iter_mut()) {
        let o = tools.0.char_indices().next().unwrap().1;
        let oponent_tool = converter.get(&o).unwrap();
        let player_tool =
            rps::get_converter_second(tools.1.char_indices().next().unwrap().1, *oponent_tool)
                .unwrap();
        score += player_tool.battle(oponent_tool);
    }
    println!("Final score (second method): {}", score);
}
fn main() {
    // Checking if the file is found.
    let mut oponent: Vec<String> = read_to_string("input")
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let mut player = oponent.clone();
    oponent.retain(|s| s == "A" || s == "B" || s == "C");
    player.retain(|s| s == "X" || s == "Y" || s == "Z");
    assert!(
        oponent.len() == player.len(),
        "Oponent and Player don't play as many turns."
    );
    first_strategy(&mut player, &oponent);
    second_strategy(&mut player, &oponent);
}
