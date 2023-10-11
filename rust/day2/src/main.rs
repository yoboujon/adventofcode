use crate::rps::get_converter;

use std::collections::HashMap;
mod rps;

fn main()
{
    let converter : HashMap<char, rps::RockPaperScissors> = rps::get_converter();
    println!("Hello world!");
}