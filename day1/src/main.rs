use std::fs::File;
use std::io::Read;
mod elves;


fn main()
{
    // Variables
    let mut vec_elf : Vec<elves::Elf> = Vec::new();
    let mut file;

    // Checking if the file is found.
    match File::open("elves") {
        Ok(actualfile) => {
            file = actualfile;
        }
        Err(fileerr) => {
            panic!("Could not open file 'elves': {}", fileerr)
        }
    };

    // There will be content if the file is read so we unwrap.
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    // Filtering the given string into multiple vectors
    let mut s = String::new();
    let mut vec_kcal: Vec<u64> = Vec::new();
    for ch in file_content.chars() {
        if ch == '\n' {
            match s.parse() {
                Ok(parsed_num) => {
                    vec_kcal.push(parsed_num);
                }
                Err(_) => {
                    vec_elf.push(elves::Elf::new(vec_kcal));
                    vec_kcal = Vec::new();
                }
            }
            s.clear();
        } else {
            if ch != '\r' {
                s.push(ch)
            }
        }
    }

    // Gathering all the total kcal onto a vector
    let mut vec_total: Vec<u64> = Vec::new();
    for elf in &vec_elf {
        vec_total.push(elf.get_total());
    }
    // Printing the max
    println!("Most Calories: {}", vec_total.iter().max().unwrap());
}
