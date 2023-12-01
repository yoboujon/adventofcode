use std::cmp;
use std::fs::read_to_string;
mod system;
use system::Directory;
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

fn first_part(disk: &Disk, max: u64) -> u64 {
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

fn second_part(disk: &Disk, disk_space: u64, update_space: u64) {
    let directories = disk.get_directories();
    let needed_space = update_space - (disk_space - disk.get_size(directories[0]));
    println!("Needed space: {}", needed_space);
    let big_directories: Vec<&&Directory> = directories
        .iter()
        .filter(|dir| disk.get_size(dir) >= needed_space)
        .collect();
    let (_, dir) = big_directories
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| disk.get_size(a).cmp(&disk.get_size(b)))
        .unwrap();
    println!(
        "Directory to delete : {},\twith size: {}",
        disk.get_path(dir),
        disk.get_size(dir)
    );
}

fn main() {
    let file_content = read_to_string("input").unwrap();
    let final_disk = decode(file_content.split("\n").collect());
    final_disk.print();
    println!("Total sum: {}", first_part(&final_disk, 100000));
    second_part(&final_disk, 70000000, 30000000);
}
