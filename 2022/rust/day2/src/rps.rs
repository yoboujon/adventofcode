use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RockPaperScissors {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl RockPaperScissors {
    pub fn battle(&self, other: &RockPaperScissors) -> u64 {
        let weight = get_weight();
        let tab = weight.get(&self).unwrap();
        (*self as u64) + 1 + (tab[*other as usize] as u64)
    }

    pub fn from_usize(value: usize) -> Option<RockPaperScissors> {
        match value {
            0 => Some(RockPaperScissors::Rock),
            1 => Some(RockPaperScissors::Paper),
            2 => Some(RockPaperScissors::Scissors),
            _ => None
        }
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

pub fn get_converter_second(ch: char, oponent: RockPaperScissors) -> Option<RockPaperScissors> {
    let binding = get_weight();
    let weight = binding.get(&oponent).unwrap();
    match ch {
        'X' => {
            let indice = weight
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(index, _)| index);
            RockPaperScissors::from_usize(indice.unwrap())
        }
        'Y' => Some(oponent),
        'Z' => {
            let indice = weight
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(index, _)| index);
            RockPaperScissors::from_usize(indice.unwrap())
        }
        _ => None,
    }
}

fn get_weight() -> HashMap<RockPaperScissors, [u8; 3]> {
    HashMap::from([
        (RockPaperScissors::Rock, [3, 0, 6]),
        (RockPaperScissors::Paper, [6, 3, 0]),
        (RockPaperScissors::Scissors, [0, 6, 3]),
    ])
}
