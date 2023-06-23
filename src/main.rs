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

    let mut analytics = Analytics::new(0, 0, 0, vec![]);

    let results = traverse(Some(dir), &mut analytics);

    print_deletions(&results.node_modules);

    println!("\n Files {}, Folders {}", results.files, results.folders);
    println!(" Total {}\n", results.total);
}
