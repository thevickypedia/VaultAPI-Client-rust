#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]
pub mod parser;
pub mod constant;
pub mod request;
pub mod decipher;

use serde_json::Value;

/// Decrypts the ciphered text into JSON object.
///
/// # Returns
/// * A `Result<Value, String>` containing deciphered content.
pub fn decrypt_vault_secret(config: parser::Config) -> Result<Value, String> {
    if config.cipher.is_empty() {
        return request::server_connection(&config);
    }
    decipher::transit_decrypt(
        &config.apikey,
        &config.cipher,
        config.transit_key_length,
        config.transit_time_bucket,
    )
}
