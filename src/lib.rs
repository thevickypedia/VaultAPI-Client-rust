mod parser;
mod constant;
mod request;
pub mod decipher;

use serde_json::Value;


pub fn decrypt_vault_secret() -> Result<Value, String> {
    let metadata = constant::build_info();
    let config = parser::arguments(&metadata);
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
