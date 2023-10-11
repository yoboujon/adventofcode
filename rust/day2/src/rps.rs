use std::collections::HashMap;

pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
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