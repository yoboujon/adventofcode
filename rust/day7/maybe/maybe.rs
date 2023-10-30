use std::cell::RefCell;

#[derive(Clone)]
pub struct File {
    size: u64,
}

impl File {
    pub fn new(s: u64) -> File {
        File { size: (s) }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(Clone)]
pub struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<RefCell<Directory>>,
    is_empty: bool,
    parent_dir: String,
}

impl Directory {
    pub fn new(n: &str, parent: &str) -> Directory {
        Directory {
            name: n.to_string(),
            files: Vec::new(),
            directories: Vec::new(),
            is_empty: true,
            parent_dir: parent.to_string(),
        }
    }

    pub fn new_file(&mut self, s: u64) {
        self.is_empty = false;
        self.files.push(File::new(s));
    }

    pub fn new_directory(&mut self, n: &str) {
        self.is_empty = false;
        self.directories.push(RefCell::new(Directory::new(n, &self.name)));
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_directory(&mut self, n: &str) -> Option<&mut RefCell<Directory>> {
        self.directories.iter_mut().find(|dir| dir.borrow().get_name() == n)
    }

    pub fn get_parent(&mut self) -> &str {
        self.parent_dir.as_str()
    }

    pub fn print(&self) {
        println!("{}", self.name);
        for f in self.files.iter() {
            println!("File: {}", f.get_size());
        }
        for d in self.directories.iter() {
            d.borrow().printlevel(1);
        }
    }

    fn printlevel(&self, level: u64) {
        let mut space = String::new();
        for _i in 0..level {
            space.push('\t');
        }
        println!("{}{}", space.clone().pop().unwrap(), self.name);
        for f in self.files.iter() {
            println!("{}->File: {}", space, f.get_size());
        }
        for d in self.directories.iter() {
            d.borrow().printlevel(level + 1);
        }
    }
}

fn _main() {
    // Root directory doesn't need a parent
    let mut root = Directory::new("/", "/");
    root.new_file(146);
    root.new_directory("toto");
    root.new_file(14323);
    let toto = root.get_directory("toto").unwrap();
    // root il existe plus
    toto.borrow().new_file(5472);
    toto.borrow().new_directory("tata");
    //let tata = toto.get_directory("tata").unwrap().borrow();
    //tata.new_file(77557);
    let root_name = root.get_name();
    toto.borrow().new_file(2426);
    println!("Toto Parent: {}", root.get_parent());
}
