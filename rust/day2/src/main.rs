use std::collections::HashMap;
mod rps;

fn main()
{
    let converter : HashMap<char, rps::RockPaperScissors> = rps::get_converter();
    let player = converter.get(&'Y').unwrap().battle(converter.get(&'A').unwrap());
    println!("First battle = {}", player);
    let player = converter.get(&'X').unwrap().battle(converter.get(&'B').unwrap());
    println!("Second battle = {}", player);
    let player = converter.get(&'Z').unwrap().battle(converter.get(&'C').unwrap());
    println!("Third battle = {}", player);
}