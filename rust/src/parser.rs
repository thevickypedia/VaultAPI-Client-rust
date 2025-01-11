use crate::constant;

const TRANSIT_KEY_LENGTH: usize = 32;
const TRANSIT_TIME_BUCKET: u64 = 60;

pub struct Config {
    pub cipher: String,
    pub apikey: String,
    pub transit_key_length: usize,
    pub transit_time_bucket: u64,
}


/// Parses and returns the command-line arguments.
///
/// # Returns
///
/// A String notion of the argument, `env_file` if present.
pub fn arguments(metadata: &constant::MetaData) -> Config {
    let args: Vec<String> = std::env::args().collect();

    let mut version = false;
    let mut env_file = String::new();
    let mut cipher = String::new();

    // Loop through the command-line arguments and parse them.
    let mut i = 1; // Start from the second argument (args[0] is the program name).
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                let helper = "VaultAPI-Client takes the arguments, --env_file and --version/-v\n\n\
                --env_file: Custom filename to load the environment variables. Defaults to '.env'\n\
                --cipher: Cipher text to decrypt\n\
                --version: Get the package version.\n".to_string();
                println!("Usage: {} [OPTIONS]\n\n{}", args[0], helper);
                std::process::exit(0)
            }
            "-V" | "-v" | "--version" => {
                version = true;
            }
            "--env_file" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    env_file = args[i].clone();
                } else {
                    println!("--env_file requires a value.");
                    std::process::exit(1)
                }
            }
            "--cipher" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    cipher = args[i].clone();
                } else {
                    println!("--cipher requires a value.");
                    std::process::exit(1)
                }
            }
            _ => {
                println!("Unknown argument: {}", args[i]);
                std::process::exit(1)
            }
        }
        i += 1;
    }
    if version {
        println!("{} {}", &metadata.pkg_name, &metadata.pkg_version);
        std::process::exit(0)
    }
    if env_file.is_empty() {
        env_file = std::env::var("env_file")
            .unwrap_or(std::env::var("ENV_FILE")
                .unwrap_or(".env".to_string()));
    }
    let env_file_path = std::env::current_dir()
        .unwrap_or_default()
        .join(env_file);
    let _ = dotenv::from_path(env_file_path.as_path());
    // Retrieve the API key from the environment
    let apikey = match std::env::var("APIKEY") {
        Ok(value) => value,
        Err(_) => {
            println!("APIKEY environment variable not set");
            std::process::exit(1)
        }
    };
    let transit_key_length = match std::env::var("TRANSMIT_KEY_LENGTH") {
        Ok(value) => value.parse::<usize>().unwrap_or(TRANSIT_KEY_LENGTH),
        Err(_) => TRANSIT_KEY_LENGTH,
    };
    let transit_time_bucket = match std::env::var("TRANSIT_TIME_BUCKET") {
        Ok(value) => value.parse::<u64>().unwrap_or(TRANSIT_TIME_BUCKET),
        Err(_) => TRANSIT_TIME_BUCKET
    };
    Config {
        cipher,
        apikey,
        transit_key_length,
        transit_time_bucket,
    }
}
