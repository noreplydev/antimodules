pub struct Config {
    pub path: String,
    pub is_help: bool,
    pub show_version: bool,
    pub ignored_folders: Vec<String>,
    pub iterare_over_hidden: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            path: String::from("."),
            is_help: false,
            show_version: false,
            ignored_folders: vec![],
            iterare_over_hidden: false,
        }
    }
}
