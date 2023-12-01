#[derive(Clone)]
pub struct File {
    size: u64,
    name: String,
}

#[derive(Clone)]
pub struct Directory {
    name: String,
    index: usize,
    files: Vec<File>,
    child: Vec<usize>,
    parent: usize,
    sublevel: u64,
}

#[derive(Clone)]
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

    pub fn get_name(&self) -> &str {
        &self.name
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
        let mut found_indexes: Vec<usize> = Vec::new();
        let found: Vec<&mut Directory> = self
            .dir_list
            .iter_mut()
            .filter(|dir| dir.name == name)
            .collect();
        if found.is_empty() {
            println!("There is no such folder '{}' in the disk.", name);
            return;
        }
        for dir in found.iter() {
            found_indexes.push(dir.index)
        }
        match self.dir_list[self.selected_dir]
            .child
            .iter()
            .find(|&index| found_indexes.contains(index))
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

    pub fn get_path(&self, dir: &Directory) -> String {
        let mut path = dir.name.to_string()+"/";
        let mut index = dir.parent;
        while index != 0 {
            path += &(self.dir_list[index].name);
            path += "/";
            index = self.dir_list[index].parent;
        }
        path.chars().rev().collect()
    }

    pub fn get_directories(&self) -> Vec<&Directory> {
        let mut dirvec: Vec<&Directory> = Vec::new();
        for dir in self.dir_list.iter() {
            dirvec.push(dir);
        }
        dirvec
    }

    pub fn get_size(&self, dir: &Directory) -> u64 {
        let mut size = 0;
        for f in &dir.files {
            size += f.size;
        }
        for index in &dir.child {
            size += self.get_size(&self.dir_list[*index]);
        }
        size
    }
}
