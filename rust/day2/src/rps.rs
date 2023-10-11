use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum RockPaperScissors {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

impl RockPaperScissors{
    pub fn battle(&self, other: &RockPaperScissors) -> u64
    {
        let weight = get_weight();
        let tab = weight.get(&self).unwrap();
        (*self as u64) + 1 + (tab[*other as usize] as u64)
    }
}

pub fn get_converter() -> HashMap<char, RockPaperScissors> {
    HashMap::from([
        ('A', RockPaperScissors::Rock),
        ('B', RockPaperScissors::Paper),
        ('C', RockPaperScissors::Scissors),
        ('X', RockPaperScissors::Rock),
        ('Y', RockPaperScissors::Paper),
        ('Z', RockPaperScissors::Scissors),
    ])
}

fn get_weight() -> HashMap<RockPaperScissors, [u8; 3]> {
    HashMap::from([
        (RockPaperScissors::Rock, [3,0,6]),
        (RockPaperScissors::Paper, [6,3,0]),
        (RockPaperScissors::Scissors, [0,6,3]),
    ])
}