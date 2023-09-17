/// 
/// AOC 2022 Day 7
///
/// Uses the indextree crate to mock the filesystem. The initial root directory is created. Each of
/// the contained files and directories are generated as the commands in the input data traverse
/// the OS. The Arena allows the root directory to link to the NodeId of other directories that get
/// created on the fly, and also track where in the operating system the user (input) currently is.
///
use std::fs;
use indextree::{Arena, NodeId};

/// 
/// FileSystem, used to track the internal state of a filesystem.
///
struct FileSystem {
    filesystem: Arena<Directory>,
    cwd: NodeId,
    root: NodeId,
}

impl FileSystem {
    /// 
    /// Create a new FileSystem instance.
    ///
    /// Stores an arena of directories, and keeps track of the current working directory.
    ///
    fn new() -> Self {
        let mut arena = Arena::new();
        let current = arena.new_node(Directory { name: "/".to_string(), files: vec![] });
        FileSystem { filesystem: arena, cwd: current, root: current }
    }
    /// 
    /// Creates a directory within the current working directory.
    ///
    fn mkdir(&mut self, new_dir: Directory) {
        let mut_arena = &mut self.filesystem;
        self.cwd.append(mut_arena.new_node(new_dir), mut_arena);
    }
    /// 
    /// Create a new file in the current working directory.
    ///
    fn touch(&mut self, new_file: File) {
        self.filesystem[self.cwd].get_mut().add_file(new_file);
    }
    /// 
    /// Change into a new directory.
    ///
    fn cd(&mut self, next: &str) {
        let mut_arena = &mut self.filesystem;
        match next {
            ".." => self.cwd = mut_arena[self.cwd].parent().unwrap(),
            oth => self.cwd = self.cwd.children(mut_arena)
                .find(|x| mut_arena[*x].get().name == oth)
                .unwrap(),
        }
    }
    /// 
    /// Get the total size of a directory.
    ///
    /// This includes both files with direct ownership, and files of indirect ownership.
    ///
    fn du(&mut self) -> u64 {
        let mut_arena = &mut self.filesystem;
        self.cwd.descendants(mut_arena).map(|x| mut_arena[x].get().get_overall_size()).sum()
    }
    /// 
    /// Get the cumulative sum of all directories with a max size cutoff.
    ///
    /// This includes both files with direct ownership, and files of indirect ownership.
    ///
    fn du_max(&mut self, max: u64) -> u64 {
        let mut_arena = &mut self.filesystem;
        self.root.descendants(mut_arena)
            .into_iter()
            .map(|x| x.descendants(mut_arena).map(|y| mut_arena[y].get().get_overall_size()).sum::<u64>())
            .filter(|x| x <= &max)
            .sum()
    }
    /// 
    /// Given a target for the required space in the operating system, find the smallest dir
    /// possible to delete to create the required space.
    ///
    fn find_smallest_deletable_dir(&mut self, target: u64) -> u64 {
        let mut_arena = &mut self.filesystem;
        self.root.descendants(mut_arena)
            .into_iter()
            .map(|x| {
                 x.descendants(mut_arena)
                    .map(|y| mut_arena[y].get().get_overall_size())
                    .sum::<u64>()
            })
            .filter(|x| x >= &target)
            .min()
            .unwrap()
    }
    ///
    /// Change dir to the root dir
    ///
    fn to_root_dir(&mut self) {
        self.cwd = self.root;
    }
}

/// 
/// Directory, used to store the contents of a directory.
///
#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
}

impl Directory {
    /// 
    /// Retrieves the overall size of this directory (files only).
    ///
    fn get_overall_size(&self) -> u64 {
        self.files.iter().map(|x| x.size).sum()
    }
    /// 
    /// Add a file to the directory.
    ///
    /// Appends a new file to the internal vector
    ///
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}

/// 
/// File struct representing the state of a file in an operating system.
///
#[derive(Debug)]
struct File {
    _name: String,
    size: u64,
}

/// 
/// Check if the input line is a command. Commands are prefixed with a $ char.
///
fn is_command(line: &str) -> bool {
    line.chars().rev().last().unwrap_or(' ') == '$'
}

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut data_iter = data.lines();
    let mut filesys = FileSystem::new();

    // we aren't interested in the first line..
    data_iter.next();
    for line in data_iter {
        if is_command(line) {
            let mut command = line.strip_prefix("$ ").unwrap().split(" ");
            match command.next().unwrap() {
                "cd" => filesys.cd(command.next().unwrap()),
                _ => (),
            }
        } else {
            let mut output = line.split(" ");
            match output.next().unwrap() {
                "dir" => filesys.mkdir( 
                    Directory { name: output.next().unwrap().to_string(), files: vec![] } 
                ),
                x => filesys.touch( 
                    File { _name: output.next().unwrap().to_string(), size: x.parse::<u64>().unwrap() } 
                ),
            }
        }
    }

    // part 1
    println!("{}", filesys.du_max(100000));

    // part 2
    filesys.to_root_dir();
    let overall_used = filesys.du();
    let overall_available = 70000000;
    let needed = 30000000;
    println!("{}", filesys.find_smallest_deletable_dir(overall_used + needed - overall_available));
}
