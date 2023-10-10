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

fn main()
{
    let elves = vec![Elf::new(vec![1000,2000,3000]),Elf::new(vec![4000]), Elf::new(vec![5000,6000]), Elf::new(vec![7000,8000,9000]), Elf::new(vec![10000])];
    for elf in &elves
    {
        println!("Elf :");
        elf.print_inventory()
    }
}
