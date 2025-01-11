mod parser;
mod constant;

use base64::{engine::general_purpose, Engine as _};
use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey};
use ring::digest;
use serde_json::Value;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const TRANSIT_TIME_BUCKET: u64 = 60; // Default time bucket
const TRANSIT_KEY_LENGTH: usize = 32; // Use 32 for AES-256

/// Decrypts a transit-encrypted payload.
///
/// # Arguments
/// * `ciphertext` - A base64-encoded encrypted string.
///
/// # Returns
/// * A `Result<Value, String>` containing the decrypted JSON payload or an error message.
pub fn transit_decrypt(ciphertext: &str) -> Result<Value, String> {
    // Retrieve the API key from the environment
    let api_key = env::var("APIKEY").map_err(|_| "APIKEY environment variable not set")?;

    // Compute the current epoch bucket
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "System time is before the UNIX epoch")?
        .as_secs()
        / TRANSIT_TIME_BUCKET;

    // Derive the AES key using SHA-256
    let hash_input = format!("{}.{}", epoch, api_key);
    let hash_output = digest::digest(&digest::SHA256, hash_input.as_bytes());
    let aes_key = &hash_output.as_ref()[..TRANSIT_KEY_LENGTH];

    // Decode the base64-encoded ciphertext
    let ciphertext_bytes = general_purpose::STANDARD.decode(ciphertext).unwrap();

    // Ensure the ciphertext is long enough
    if ciphertext_bytes.len() < 12 {
        return Err("Ciphertext is too short".into());
    }

    // Extract the nonce (first 12 bytes) and the actual encrypted data
    let (nonce_bytes, encrypted_data) = ciphertext_bytes.split_at(12);

    // Initialize AES-GCM decryption
    let unbound_key = UnboundKey::new(&aead::AES_256_GCM, aes_key)
        .map_err(|_| "Failed to create AES key")?;
    let key = LessSafeKey::new(unbound_key);
    let nonce = Nonce::try_assume_unique_for_key(nonce_bytes)
        .map_err(|_| "Failed to create nonce")?;

    // Decrypt the data
    let mut binding = encrypted_data.to_vec();
    let decrypted_data = key.open_in_place(nonce, Aad::empty(), &mut binding).unwrap();

    // Parse the decrypted data as JSON
    let decrypted_json: Value = serde_json::from_slice(decrypted_data)
        .map_err(|_| "Failed to parse decrypted data as JSON")?;

    Ok(decrypted_json)
}


fn main() {
    let ciphertext = "CIPHER_TEXT_SHOULD_BE_HERE";
    let decipherd = transit_decrypt(ciphertext).expect("Transit decryption failed");
    println!("{}", decipherd);
}
