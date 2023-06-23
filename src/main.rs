mod util;

use util::explore::*;
use util::structs::*;

fn main() {
    // get arguments and check if a file was passed
    // otherwise use pwd and continue

    let mut analytics = Analytics::new(0, 0, 0, 0);

    let results = traverse(Some("./"), &mut analytics);

    println!("\n Files {}, Folders {}", results.files, results.folders);
    println!(" Total {}", results.total);
    println!("\n Deleted node_modules {}\n", results.node_modules);
}
