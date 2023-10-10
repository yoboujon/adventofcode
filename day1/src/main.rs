use std::fs::File;
use std::io::Read;
mod elves;


fn main()
{
    let elf_vec = vec![elves::Elf::new(vec![1000,2000,3000]),elves::Elf::new(vec![4000]), elves::Elf::new(vec![5000,6000]), elves::Elf::new(vec![7000,8000,9000]), elves::Elf::new(vec![10000])];
    for elf in &elf_vec
    {
        println!("Elf :");
        elf.print_inventory()
    }

    let mut file;
    // Checking if the file is found.
    match File::open("elves") {
        Ok(actualfile) => {
            file = actualfile;
        }
        Err(fileerr) => {
            panic!("Could not open file 'elves' : {}", fileerr)
        }
    };

    // There will be content if the file is read so we unwrap.
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    println!("{}", file_content);
}
