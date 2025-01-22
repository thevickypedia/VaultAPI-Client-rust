#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]
pub mod routes;
pub mod constant;
pub mod decipher;
mod enums;
pub mod parser;
pub mod request;
mod util;

use serde_json::Value;

/// Decrypts the ciphered text into JSON object.
///
/// # Arguments
/// * `arg_config` - Config object to retrieve CLI arguments.
///
/// # Returns
/// * A `Result<Value, String>` containing deciphered content.
pub fn decrypt_vault_secret(arg_config: parser::ArgConfig) -> Result<Value, String> {
    let env_config = parser::env_variables();
    if arg_config.cipher.is_empty() {
        return request::server_connection(&arg_config, &env_config);
    }
    decipher::transit_decrypt(
        &env_config.apikey,
        &env_config.secret,
        &arg_config.cipher,
        env_config.transit_key_length,
        env_config.transit_time_bucket,
    )
}
