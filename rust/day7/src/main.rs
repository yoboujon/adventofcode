use std::fs::read_to_string;
mod system;
use system::Disk;

fn decode(cmd: Vec<&str>) -> Disk {
    let mut index = 1; // ignoring the first cd /
    let mut disk = Disk::new();
    while index < cmd.len() {
        let mut info: Vec<&str> = cmd[index].split(' ').collect();
        if info[1] == "ls" {
            index += 1;
            while (index < cmd.len()) && (!(cmd[index].as_bytes()[0] == ('$' as u8))) {
                info = cmd[index].split(' ').collect();
                match info[0] {
                    "dir" => disk.create_directory(info[1]),
                    &_ => disk.create_file(info[0].parse::<u64>().unwrap(), info[1]),
                }
                index += 1;
            }
        } else if info[1] == "cd" {
            match info[2] {
                ".." => disk.goto_parent(),
                &_ => disk.change_directory(info[2]),
            }
            index += 1;
        } else {
            index += 1;
        }
    }
    disk
}

fn list_dirs(disk: &Disk, max: u64) -> u64 {
    let mut sum = 0;
    for iter in disk
        .get_directories()
        .iter()
        .filter(|dir| disk.get_size(dir) <= max)
    {
        println!(
            "dir: {} with size: {}",
            disk.get_path(iter),
            disk.get_size(iter)
        );
        sum += disk.get_size(iter);
    }
    sum
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    let final_disk = decode(file_content.split("\n").collect());
    final_disk.print();
    println!("Total sum: {}", list_dirs(&final_disk, 100000));
}
