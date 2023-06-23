mod util;

use std::env;
use util::explore::*;
use util::structs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dir = "./";

    if args.len() > 1 {
        dir = args[1].as_str()
    }

    let mut analytics = Analytics::new(0, 0, 0, 0);

    let results = traverse(Some(dir), &mut analytics);

    println!("\n Files {}, Folders {}", results.files, results.folders);
    println!(" Total {}", results.total);
    println!("\n Deleted node_modules {}\n", results.node_modules);
}
