use std::fs::read_to_string;
mod system;

fn decode(cmd: Vec<&str>) -> system::Disk {
    let mut index = 1;  // ignoring the first cd /
    let mut disk = system::Disk::new();
    while index < cmd.len() {
        if cmd[index].contains("ls") {
            index +=1;
            while (index < cmd.len()) && !(cmd[index].contains("ls") || cmd[index].contains("cd")) {
                let info : Vec<&str> = cmd[index].split(' ').collect();
                match info[0] {
                    "dir" => disk.create_directory(info[1]),
                    &_ => disk.create_file(info[0].parse::<u64>().unwrap(),info[1])
                }
                index +=1;
            }
        } else if cmd[index].contains("cd") {
            let info : Vec<&str> = cmd[index].split(' ').collect();
            match info[2] {
                ".." => disk.goto_parent(),
                &_ => disk.change_directory(info[2]) 
            }
            index +=1;
        } else {
            index += 1;
        }
    }
    disk
}

fn main() {
    let file_content = read_to_string("exemple").unwrap();
    let final_disk = decode(file_content.split("\n").collect());
    final_disk.print();
}
