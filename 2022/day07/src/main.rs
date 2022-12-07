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
            self.size += d.borrow_mut().calculate_size();
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
            if d.borrow().name == name {
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
            sum_of_sizes += d.borrow().find_sum_dirs(at_most);
        }

        sum_of_sizes
    }

    fn find_smallest_dir_at_least(&self, at_least: u64) -> Option<u64> {
        let mut smallest_dir_size = None;

        if self.size >= at_least {
            smallest_dir_size = Some(self.size);
        }

        for d in &self.subdirectories {
            let subdir_size = d.borrow().find_smallest_dir_at_least(at_least);
            if subdir_size.is_some() {
                if smallest_dir_size.is_none() {
                    smallest_dir_size = subdir_size;
                } else {
                    if subdir_size.unwrap() < smallest_dir_size.unwrap() {
                        smallest_dir_size = subdir_size;
                    }
                }
            }
        }

        smallest_dir_size
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

    root.borrow_mut().calculate_size();

    let sum_of_dir_sizes = root.borrow().find_sum_dirs(100000);

    println!("Sum of dirs at most 100000 is {}", sum_of_dir_sizes);

    let total_space: u64 = 70000000;
    let needed_space: u64 = 30000000;
    let used_space = root.borrow().size;
    let free_space = total_space - used_space;

    if free_space >= needed_space {
        println!("No need to free any space");
    } else {
        root.borrow().find_smallest_dir_at_least(needed_space - free_space)
            .map_or_else(
                || println!("Can't find the directory"),
                |s| println!("Smallest space to be freed is {}", s)
            );
    }
}
