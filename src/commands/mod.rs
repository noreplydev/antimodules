use crate::models::config::Config;

const FLAGS: [&str; 6] = ["--ignore", "-i", "--help", "-h", "--version", "-v"];

pub fn set_config(vec: Vec<String>) -> Config {
    let mut config = Config::new();

    for i in 0..vec.len() {
        match vec[i].as_str() {
            "--ignore" | "-i" => {
                if i + 1 < vec.len() {
                    config.ignored_folders.push(vec[i + 1].clone());
                }
            }
            "--help" | "-h" => config.is_help = true,
            "--version" | "-v" => config.show_version = true,
            _ => {}
        }
    }

    // check if the last argument or the panultimate argument are a flag
    // if there aren't, then the last argument is the path
    let is_command = FLAGS.contains(&vec[vec.len() - 1].as_str())
        || FLAGS.contains(&vec[vec.len() - 2].as_str());

    if !is_command {
        config.path = vec[vec.len() - 1].clone();
    }

    check_config(&config);

    return config;
}

fn check_config(config: &Config) {
    if config.is_help {
        println!("\nUsage: antimodules [options] [path]");
        println!("\nOptions:");
        println!(
            "  -i, --ignore <folder,...>  Ignore a folders to iterate over. Separate with comma."
        );
        println!("  -h, --help             Display this message");
        println!("  -v, --version          Display version number\n");

        std::process::exit(0);
    }

    if config.show_version {
        println!("\nAntimodules v1.0.0\n");
        std::process::exit(0);
    }
}
