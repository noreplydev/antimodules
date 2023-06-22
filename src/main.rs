use std::fs;

fn main() {
    // get arguments and check if a file was passed
    // otherwise use pwd and continue
    traverse(Some("./"))
}

fn traverse(dir: Option<&str>) {
    if dir == None {
        return;
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
            traverse(Some(path));
        } else {
            println!("{}", path);
        }
    }
}
