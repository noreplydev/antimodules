mod commands;
mod models;
mod util;

use commands::set_config;
use models::analytics::Analytics;
use std::env;
use util::{print_deletions, traverse};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = set_config(args);

    let mut analytics = Analytics::new(0, 0, 0, vec![]);
    let results = traverse(Some(config.path.as_str()), &mut analytics);

    print_deletions(&results.node_modules);

    println!("\n Files {}, Folders {}", results.files, results.folders);
    println!(" Total {}\n", results.total);
}
