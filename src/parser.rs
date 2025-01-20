use reqwest::Url;
use crate::constant;

const TRANSIT_KEY_LENGTH: usize = 32;
const TRANSIT_TIME_BUCKET: u64 = 60;

pub struct EnvConfig {
    pub vault_server: Url,
    pub apikey: String,
    pub secret: String,
    pub transit_key_length: usize,
    pub transit_time_bucket: u64,
}

pub struct ArgConfig {
    pub cipher: String,
    pub table_name: String,
    pub get_secret: String,
    pub get_secrets: String,
    pub get_table: String,
}


fn get_env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(value) => value,
        Err(_) => {
            if !default.is_empty() {
                return default.to_string();
            }
            println!("{:} environment variable not set", key);
            std::process::exit(1)
        }
    }
}

pub fn load_env(env_file: &String) {
    let env_file_path = std::env::current_dir()
        .unwrap_or_default()
        .join(env_file);
    let _ = dotenv::from_path(env_file_path.as_path());
}

pub fn default_env_file() -> String {
    std::env::var("env_file")
        .unwrap_or(std::env::var("ENV_FILE")
            .unwrap_or(".env".to_string()))
}

pub fn env_variables() -> EnvConfig {
    load_env(&default_env_file());
    // Retrieve the API key from the environment
    let apikey = get_env("APIKEY", "");
    let secret = get_env("SECRET", "");
    let vault_server_env = get_env("VAULT_SERVER", "");
    let vault_server = match Url::parse(&vault_server_env) {
        Ok(url) => url,
        Err(_e) => panic!("Failed to parse vault address"),
    };
    let transit_key_length = match std::env::var("TRANSMIT_KEY_LENGTH") {
        Ok(value) => value.parse::<usize>().unwrap_or(TRANSIT_KEY_LENGTH),
        Err(_) => TRANSIT_KEY_LENGTH,
    };
    let transit_time_bucket = match std::env::var("TRANSIT_TIME_BUCKET") {
        Ok(value) => value.parse::<u64>().unwrap_or(TRANSIT_TIME_BUCKET),
        Err(_) => TRANSIT_TIME_BUCKET
    };
    EnvConfig {
        vault_server,
        apikey,
        secret,
        transit_key_length,
        transit_time_bucket,
    }
}


/// Parses and returns the command-line arguments and environment variables.
///
/// # Returns
/// A String notion of the argument, `env_file` if present.
pub fn arguments(metadata: &constant::MetaData) -> ArgConfig {
    let args: Vec<String> = std::env::args().collect();

    let mut version = false;
    let mut env_file = String::new();
    let mut cipher = String::new();
    let mut table_name = String::new();
    let mut get_secret = String::new();
    let mut get_secrets = String::new();
    let mut get_table = String::new();

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
            "--table" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    table_name = args[i].clone();
                } else {
                    println!("--get-secret requires a value.");
                    std::process::exit(1)
                }
            }
            "--get-secret" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    get_secret = args[i].clone();
                } else {
                    println!("--get-secret requires a value.");
                    std::process::exit(1)
                }
            }
            "--get-secrets" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    get_secrets = args[i].clone();
                } else {
                    println!("--get-secrets requires a value.");
                    std::process::exit(1)
                }
            }
            "--get-table" => {
                i += 1; // Move to the next argument.
                if i < args.len() {
                    get_table = args[i].clone();
                } else {
                    println!("--get-table requires a value.");
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
        env_file = default_env_file();
    }
    load_env(&env_file);
    ArgConfig {
        cipher,
        table_name,
        get_secret,
        get_secrets,
        get_table,
    }
}
