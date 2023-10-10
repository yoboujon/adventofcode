pub struct Elf {
    inventory : Vec<u64>,
}

impl Elf {
    pub fn new(vec : Vec<u64>) -> Elf {
        Elf {
            inventory: vec,
        }
    }

    pub fn get_total(& self) -> u64
    {
        self.inventory.iter().sum()
    }
}