
fn main() {
    let metadata = vaultapi::constant::build_info();
    let config = vaultapi::parser::arguments(&metadata);
    match vaultapi::decrypt_vault_secret(config) {
        Ok(value) => {
            println!("{}", value);
        },
        Err(err) => {
            println!("{}", err);
        }
    };
}
