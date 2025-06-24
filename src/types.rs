#[derive(Debug, PartialEq)]
pub enum View {
    PathList,
    DirectoryContents,
}

#[derive(Debug, Clone)]
pub enum EntryInfo {
    SymlinkTarget(String),
    Error(String),
    None,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub is_symlink: bool,
    pub info: EntryInfo,
}

#[derive(Debug)]
pub struct DirectoryView {
    pub current_path: String,
    pub directory_contents: Vec<FileEntry>,
    pub filtered_contents: Vec<FileEntry>,
    pub search_mode: bool,
    pub search_query: String,
}

impl DirectoryView {
    pub fn new(path: String) -> Self {
        Self {
            current_path: path,
            directory_contents: Vec::new(),
            filtered_contents: Vec::new(),
            search_mode: false,
            search_query: String::new(),
        }
    }

    pub fn get_display_contents(&self) -> &[FileEntry] {
        if self.search_query.is_empty() {
            &self.directory_contents
        } else {
            &self.filtered_contents
        }
    }

    pub fn clear(&mut self) {
        self.current_path.clear();
        self.directory_contents.clear();
        self.filtered_contents.clear();
        self.search_mode = false;
        self.search_query.clear();
    }
}