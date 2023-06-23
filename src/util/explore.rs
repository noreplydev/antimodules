use super::structs::Analytics;
use std::fs;

pub fn traverse(dir: Option<&str>, analytics: &mut Analytics) -> Analytics {
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
            println!("\r{}", path);
            Analytics::add_file(analytics);
            continue;
        }

        // In case of a folder
        Analytics::add_folder(analytics);

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

        if file.file_type().unwrap().is_dir() {
            traverse(Some(path), analytics);
        }
    }

    return Analytics::new(
        analytics.files,
        analytics.folders,
        analytics.total,
        analytics.node_modules,
    );
}
