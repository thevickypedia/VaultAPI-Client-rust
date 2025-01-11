use vaultapi::decrypt_vault_secret;

fn main() {
    match decrypt_vault_secret() {
        Ok(value) => {
            println!("{}", value);
        },
        Err(err) => {
            println!("{}", err);
        }
    };
}
