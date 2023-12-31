use crate::models::{analytics::Analytics, config::Config};
use std::fs;

pub fn traverse(dir: Option<&str>, analytics: &mut Analytics, config: &Config) -> Analytics {
    let files = fs::read_dir(dir.unwrap());

    let files = match files {
        Ok(files) => files,
        Err(e) => {
            panic!(
                "Something went wrong traversing the directory '{}': {}",
                dir.unwrap(),
                e
            )
        }
    };

    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let path = path.to_str().unwrap();

        // In case of a file
        if !file.file_type().unwrap().is_dir() {
            println!("{}", path);
            Analytics::add_file(analytics);
            continue;
        }

        // In case of a folder
        Analytics::add_folder(analytics);

        let dir_name = file.file_name();
        let dir_name = dir_name.to_str().unwrap();

        if config.ignored_folders.contains(&dir_name.to_string()) {
            continue;
        }

        if dir_name == "node_modules" {
            let mut new_node_modules = vec![path.to_string()];
            Analytics::add_node_modules(analytics, &mut new_node_modules);
            let remove_status = fs::remove_dir_all(path);

            match remove_status {
                Ok(_) => {
                    println!("\x1b[31m - {} \x1b[0m", path);
                }
                Err(e) => {
                    println!("Error removing {}: {}", path, e);
                }
            }

            continue;
        }

        if file.file_type().unwrap().is_dir() {
            traverse(Some(path), analytics, config);
        }
    }

    return Analytics::new(
        analytics.files,
        analytics.folders,
        analytics.total,
        analytics.node_modules.clone(),
    );
}

pub fn print_deletions(node_modules: &Vec<String>) {
    if node_modules.len() < 1 {
        println!("\n 0 node_modules");
        return;
    }

    println!("\n  {} Deleted element(s)", node_modules.len());
    for folder in node_modules {
        println!("   - {}", folder);
    }
}
