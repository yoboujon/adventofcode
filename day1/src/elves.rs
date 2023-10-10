pub struct Elf {
    inventory : Vec<u64>,
}

impl Elf {
    pub fn new(vec : Vec<u64>) -> Elf {
        Elf {
            inventory: vec,
        }
    }

    pub fn print_inventory(& self)
    {
        for inv in &self.inventory {
            println!("{}",inv)
        }
    }
}