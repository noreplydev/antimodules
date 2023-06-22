use std::fs;
use std::io::Write;

struct Analytics {
    files: usize,
    folders: usize,
    total: usize,
    node_modules: usize,
}

impl Analytics {
    fn new(files: usize, folders: usize, total: usize, node_modules: usize) -> Analytics {
        Analytics {
            files,
            folders,
            total,
            node_modules,
        }
    }

    fn add_file(&mut self) {
        self.files += 1;
        self.total += 1;
    }

    fn add_folder(&mut self) {
        self.folders += 1;
        self.total += 1;
    }

    fn add_node_modules(&mut self) {
        self.node_modules += 1;
    }
}

fn main() {
    // get arguments and check if a file was passed
    // otherwise use pwd and continue

    let mut analytics = Analytics::new(0, 0, 0, 0);

    let results = traverse(Some("./"), &mut analytics);

    println!("\n Files {}, Folders {},", results.files, results.folders);
    println!(" Node Modules {}", results.node_modules);
    println!("\n Total {}\n", results.total);
}

fn traverse(dir: Option<&str>, analytics: &mut Analytics) -> Analytics {
    if dir == None {
        return Analytics::new(
            analytics.files,
            analytics.folders,
            analytics.total,
            analytics.node_modules,
        );
    }

    let files = fs::read_dir(dir.unwrap());

    let files = match files {
        Ok(files) => files,
        Err(e) => {
            panic!(
                "Something went wrong traversing the directory <{}>: {}",
                dir.unwrap(),
                e
            )
        }
    };

    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let path = path.to_str().unwrap();

        if file.file_type().unwrap().is_dir() {
            let file_name = file.file_name();
            let file_name = file_name.to_str().unwrap();

            if file_name == "node_modules" {
                Analytics::add_node_modules(analytics);
                let remove_status = fs::remove_dir_all(path);

                match remove_status {
                    Ok(_) => {
                        println!("Removed {}", path);
                    }
                    Err(e) => {
                        println!("Error removing {}: {}", path, e);
                    }
                }

                continue;
            }

            traverse(Some(path), analytics);
        } else {
            println!("\r{}", path);
            std::io::stdout().flush();
        }
    }

    return Analytics::new(
        analytics.files,
        analytics.folders,
        analytics.total,
        analytics.node_modules,
    );
}
