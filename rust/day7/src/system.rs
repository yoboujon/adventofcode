#[derive(Debug)]
pub struct File {
    size: u64,
    name: String,
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    index: usize,
    files: Vec<File>,
    child: Vec<usize>,
    parent: usize,
    sublevel: u64,
}

pub struct Disk {
    dir_list: Vec<Directory>,
    selected_dir: usize,
}

impl Directory {
    pub fn new(n: &str, index: usize, parent: usize, sublevel: u64) -> Directory {
        Directory {
            name: n.to_string(),
            index: index,
            files: Vec::new(),
            child: Vec::new(),
            parent: parent,
            sublevel: sublevel,
        }
    }

    pub fn create_file(&mut self, size: u64, name: &str) {
        self.files.push(File {
            size: size,
            name: name.to_string(),
        });
    }
}

impl Disk {
    pub fn new() -> Disk {
        let mut disk = Disk {
            dir_list: Vec::new(),
            selected_dir: 0,
        };
        disk.dir_list.push(Directory::new("/", 0, 0, 0));
        disk
    }

    pub fn create_file(&mut self, size: u64, name: &str) {
        self.dir_list[self.selected_dir].create_file(size, name);
    }

    pub fn create_directory(&mut self, name: &str) {
        let index = self.dir_list.len();
        self.dir_list[self.selected_dir].child.push(index);
        self.dir_list.push(Directory::new(
            name,
            self.dir_list.len(),
            self.selected_dir,
            self.dir_list[self.selected_dir].sublevel + 1,
        ));
    }

    pub fn change_directory(&mut self, name: &str) {
        let found_index: usize;
        match self.dir_list.iter_mut().find(|dir| dir.name == name) {
            None => {
                println!("There is no such folder '{}' in the disk.", name);
                return;
            }
            Some(found_dir) => found_index = found_dir.index,
        }
        match self.dir_list[self.selected_dir]
            .child
            .iter()
            .find(|&&index| index == found_index)
        {
            None => println!(
                "There is no such folder '{}' in the directory '{}'",
                name, self.dir_list[self.selected_dir].name
            ),
            Some(&found_index) => self.selected_dir = found_index,
        }
    }

    pub fn goto_parent(&mut self) {
        self.selected_dir = self.dir_list[self.selected_dir].parent;
    }

    pub fn print(&self) {
        let root = &self.dir_list[0];
        self.printdir(root);
    }

    fn printdir(&self, dir: &Directory) {
        let mut space = String::new();
        for _ in 0..dir.sublevel {
            space.push('\t');
        }
        println!("{}{} (dir)", space, dir.name);
        for d in &dir.child {
            self.printdir(&self.dir_list[*d])
        }
        for f in dir.files.iter() {
            println!("{}\tFile: {}, size: {}", space, f.name, f.size);
        }
    }
}
