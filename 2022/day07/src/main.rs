use std::cell::RefCell;
use std::io;
use std::rc::Rc;

#[derive(Debug)]
struct Directory {
    name: String,
    size: u64,
    subdirectories: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

impl Directory {
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name.to_owned(),
            size: 0,
            subdirectories: Vec::new(),
            files: Vec::new(),
        }))
    }

    fn calculate_size(&mut self) -> u64 {
        if self.size != 0 {
            return self.size;
        }

        for d in &mut self.subdirectories {
            let mut d1 = d.borrow_mut();
            self.size += d1.calculate_size();
        }

        for f in &self.files {
            self.size += f.size;
        }

        self.size
    }

    fn handle_ls_output_line(&mut self, output_line: &str) {
        let (size_or_dir, name) = output_line.split_once(' ').unwrap();

        if size_or_dir == "dir" {
            self.subdirectories.push(Directory::new(name));
        } else {
            let size = u64::from_str_radix(size_or_dir, 10).unwrap();
            self.files.push(File::new(name, size));
        }
    }

    fn handle_cd(&self, name: &str) -> Option<Rc<RefCell<Directory>>> {
        for d in &self.subdirectories {
            let d1 = d.borrow();
            if d1.name == name {
                return Some(d.clone());
            }
        }
        None
    }

    fn find_sum_dirs(&self, at_most: u64) -> u64 {
        let mut sum_of_sizes: u64 = 0;

        if self.size <= at_most {
            sum_of_sizes += self.size;
        }

        for d in &self.subdirectories {
            let d1 = d.borrow();
            sum_of_sizes += d1.find_sum_dirs(at_most);
        }

        sum_of_sizes
    }
}

impl File {
    fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_owned(),
            size,
        }
    }
}

fn main() {
    let mut directory_stack = Vec::new();
    let root = Directory::new("/");

    for line in io::stdin().lines() {
        let input = line.unwrap();

        let (hash_or_else, cmd) = input.split_once(' ').unwrap();

        if hash_or_else == "$" {
            if cmd == "ls" {
                continue; // nothing to do here
            } else {
                let (_, dir) = cmd.split_once(' ').unwrap();

                if dir == "/" {
                    directory_stack.clear();
                    directory_stack.push(root.clone());
                } else if dir == ".."{
                    directory_stack.pop();
                } else {
                    let current_dir = directory_stack.last().unwrap();
                    let subdir = current_dir.borrow().handle_cd(dir).unwrap();
                    directory_stack.push(subdir);
                }
            }
        } else {
            // handle ls output
            let current_dir = directory_stack.last().unwrap();
            let mut current_mut_dir = current_dir.borrow_mut();
            current_mut_dir.handle_ls_output_line(&input);
        }
    }

    let mut root_mut = root.borrow_mut();
    root_mut.calculate_size();
    drop(root_mut);

    let root1 = root.borrow();
    let sum_of_dir_sizes = root1.find_sum_dirs(100000);

    println!("Sum of dirs at most 100000 is {}", sum_of_dir_sizes);
}
