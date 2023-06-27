use crate::models::config::Config;

const FLAGS: [&str; 6] = ["--ignore", "-i", "--help", "-h", "--version", "-v"];

pub fn set_config(vec: Vec<String>) -> Config {
    let mut config = Config::new();

    for i in 0..vec.len() {
        match vec[i].as_str() {
            "--ignore" | "-i" => {
                if i + 1 < vec.len() {
                    config.ignored_folders = vec[i + 1].split(',').map(|s| s.to_string()).collect();
                } else {
                    panic!("--ignore flag must be followed by a folder name")
                }
            }
            "--help" | "-h" => config.is_help = true,
            "--version" | "-v" => config.show_version = true,
            _ => {}
        }
    }

    // check if the last argument or the panultimate argument are a flag
    // if there aren't, then the last argument is the path
    if vec.len() >= 2 && !FLAGS.contains(&vec[vec.len() - 2].as_str()) {
        if FLAGS.contains(&vec[vec.len() - 3].as_str()) {
            config.path = vec[vec.len() - 1].clone();
        }
    }

    check_config(&config);

    println!("{}", config.path);
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
        println!("                 _    _                        _         _");
        println!("                | |  (_)                      | |       | |");
        println!("    __ _  _ __  | |_  _  _ __ ___    ___    __| | _   _ | |  ___  ___");
        println!("   / _` || '_ \\ | __|| || '_ ` _ \\  / _ \\  / _` || | | || | / _ \\/ __|");
        println!("  | (_| || | | || |_ | || | | | | || (_) || (_| || |_| || ||  __/\\__ \\");
        println!("   \\__,_||_| |_| \\__||_||_| |_| |_| \\___/  \\__,_| \\__,_||_| \\___||___/");
        println!("\n                                                               v1.0.0\n");
        std::process::exit(0);
    }
}
