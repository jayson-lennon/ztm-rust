enum EntryKind {
    Directory,
    File,
}

struct DirectoryEntry {
    name: String,
    size: usize, // bytes
    kind: EntryKind,
}

impl DirectoryEntry {
    pub fn new_dir(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            kind: EntryKind::Directory,
        }
    }
    pub fn new_file(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            kind: EntryKind::File,
        }
    }
}

enum DisplayMode {
    Brief,
    Detailed,
}

fn formatted_entry(entry: &DirectoryEntry, mode: &DisplayMode) -> String {
    match mode {
        DisplayMode::Brief => match entry.kind {
            EntryKind::Directory => format!("{name}/", name = entry.name),
            EntryKind::File => format!("{name}", name = entry.name),
        },
        DisplayMode::Detailed => match entry.kind {
            EntryKind::Directory => {
                format!(
                    "DIR  {size:8} {name}/",
                    size = entry.size,
                    name = entry.name
                )
            }
            EntryKind::File => {
                format!("FILE {size:8} {name}", size = entry.size, name = entry.name)
            }
        },
    }
}

fn main() {
    let display_mode = DisplayMode::Brief;
    // let display_mode = DisplayMode::Detailed;

    // our sample directory entries
    let entries = vec![
        DirectoryEntry::new_dir("dir-1", 0),
        DirectoryEntry::new_dir("dir-2", 0),
        DirectoryEntry::new_file("file-1", 1000),
        DirectoryEntry::new_file("file-2", 2000),
        DirectoryEntry::new_file("file-3", 3000),
        DirectoryEntry::new_file("file-4", 4000),
    ];

    // show a heading when using detailed display mode
    match display_mode {
        DisplayMode::Detailed => {
            println!("kind size     name");
            println!("==== ======== ====");
        }
        _ => (),
    }

    for entry in entries {
        let formatted = formatted_entry(&entry, &display_mode);
        println!("{formatted}");
    }
}
