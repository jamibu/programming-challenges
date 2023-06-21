use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;


#[derive(Debug, Default)]
struct FileSystem {
    folders: Vec<Folder>,
    files: Vec<FileData>,
}


impl FileSystem {
    fn new_with_root() -> FileSystem {
        let root = Folder::new("/".to_string(), 0, None);
        let mut folders = vec!();
        folders.push(root);
        FileSystem { folders, files: vec!()}
    }

    fn add_folder(&mut self, name: String, parent: usize) -> usize {
        let idx = self.folders.len();
        let node = Folder::new(name, self.folders.len(), Some(parent));
        self.folders.push(node);
        return idx;
    }

    fn get_parent(&self, current_folder: usize) -> usize {
        return self.folders[current_folder].parent
            .expect("Can't go back already at root.");
    }

    fn add_file(&mut self, current_folder: usize, name: String, size: i32) {
        let idx = self.files.len();
        self.files.push(FileData{idx, name, size, parent: current_folder});
        self.folders[current_folder].files.push(idx);
    }
}



#[derive(Debug, PartialEq)]
struct Folder {
    idx: usize,
    name: String,
    parent: Option<usize>,
    files: Vec<usize>,
    children: Vec<usize>,
}


impl Folder {
    fn new(name: String, idx: usize, parent: Option<usize>) -> Folder {
        return Folder { idx, name, parent, files: vec!(), children: vec!() }
    }
}


#[derive(Debug, PartialEq)]
struct FileData {
    idx: usize,
    name: String,
    size: i32,
    parent: usize
}


enum Cmd {
    Skip,
    CdPrev,
    CdFolder(String),
    LsFile(i32, String),
}


fn parse_cmd(cmd_string: String) -> Cmd {
    let cmd: Cmd;
    if cmd_string == "" || cmd_string.starts_with("$ ls") || cmd_string.starts_with("dir ") {
        cmd = Cmd::Skip; 
    }  else if cmd_string.starts_with("$ cd ..") {
        cmd = Cmd::CdPrev;
    } else if cmd_string.starts_with("$ cd") {
        let dir = cmd_string.split_whitespace().last().unwrap();
        cmd = Cmd::CdFolder(dir.to_string());
    } else {
        let (size, name) = cmd_string.split_once(" ").unwrap();
        let size: i32 = size.parse().unwrap();
        cmd = Cmd::LsFile(size, name.to_string());
    }
    return cmd;
}


fn part1(folder_sizes: &HashMap<usize, i32>) {
    let mut answer: i32 = 0;
    for (_key, value) in folder_sizes {
        if value <= &100000 {
            answer += value
        }
    }

    println!("Part1: {}", answer);
}


fn part2(folder_sizes: &HashMap<usize, i32>, filesystem: &FileSystem) {
    let target: i32 = 30000000;
    let total: i32 = 70000000;

    let freeup: i32 = target - (total - folder_sizes[&0]);
    println!("Free up: {}", freeup);

    let mut min = i32::MAX;
    let mut min_idx: usize = 0;
    for (key, value) in folder_sizes {
        if value >= &freeup && value < &min {
            min = *value;
            min_idx = *key;
        }
    }

    println!("Part 2: {} - {}", filesystem.folders[min_idx].name, min);
}


fn main() {
    // let filename = "example.txt";
    let filename = "puzzle_input.txt";
    let reader = BufReader::new(File::open(filename)
        .expect("Cannot open file"));

    let mut filesystem: FileSystem = FileSystem::new_with_root();
    let mut current_folder = 0;

    for line in reader.lines().skip(1) {
        let cmd = parse_cmd(line.unwrap());

        match cmd {
            Cmd::Skip => continue,
            Cmd::CdPrev => current_folder = filesystem.get_parent(current_folder),
            Cmd::CdFolder(name) => current_folder = filesystem.add_folder(name, current_folder),
            Cmd::LsFile(size, name) => filesystem.add_file(current_folder, name, size)
        }
    }

    let mut folder_sizes: HashMap<usize, i32> = HashMap::new();

    for filedata in &filesystem.files {
        let mut parent_folder = Some(filedata.parent);
        while let Some(idx) = parent_folder {
            *folder_sizes.entry(idx).or_insert(0) += filedata.size;
            parent_folder = filesystem.folders[idx].parent;
        }
    }

    part1(&folder_sizes);
    part2(&folder_sizes, &filesystem);
}
