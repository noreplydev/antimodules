pub struct Analytics {
    pub files: usize,
    pub folders: usize,
    pub total: usize,
    pub node_modules: Vec<String>,
}

impl Analytics {
    pub fn new(files: usize, folders: usize, total: usize, node_modules: Vec<String>) -> Analytics {
        Analytics {
            files,
            folders,
            total,
            node_modules,
        }
    }

    pub fn add_file(&mut self) {
        self.files += 1;
        self.total += 1;
    }

    pub fn add_folder(&mut self) {
        self.folders += 1;
        self.total += 1;
    }

    pub fn add_node_modules(&mut self, item: &mut Vec<String>) {
        self.node_modules.append(item)
    }
}
