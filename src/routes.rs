use crate::enums::Method;
use crate::parser::EnvConfig;
use crate::{enums, parser, request, util};
use serde_json::{to_value, Value};
use std::collections::HashMap;
use std::process::exit;

/// Function to generate a table request, with just the table name as query param.
///
/// # Arguments
/// * `url` - API endpoint Url.
/// * `method` - Method enum.
/// * `table_name` - Table name.
/// * `env_config` - Environment variables' configuration.
///
/// # Returns
/// * A `PreparedRequest` struct containing the request essentials.
fn table_request(
    url: String,
    method: Method,
    table_name: &String,
    env_config: &EnvConfig,
) -> request::PreparedRequest {
    request::PreparedRequest {
        url: url.to_string(),
        method,
        params: HashMap::from([("table_name".to_string(), table_name.to_string())]),
        payload: HashMap::new(),
        headers: request::auth_headers(&env_config.apikey),
    }
}

/// Retrieve secret(s) from the server.
///
/// # Arguments
/// * `key` - Secret key for the which the value has to be retrieved.
/// * `table_name` - Table name where the secret is stored.
///
/// # Returns
/// * A `Result<Value, String>` object with decrypted payload.
pub fn get_secret(key: &String, table_name: &String) -> Result<Value, String> {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::GetSecret.as_str(),
    ]);
    let params = HashMap::from([
        ("table_name".to_string(), table_name.to_string()),
        ("key".to_string(), key.to_string()),
    ]);
    let request = request::PreparedRequest {
        url,
        method: Method::Get,
        params,
        payload: HashMap::new(),
        headers: request::auth_headers(&env_config.apikey),
    };
    let response = request::make_request(request);
    request::decrypt_response(&env_config, &response)
}

/// Retrieve ALL the secrets stored in a particular table from the server.
///
/// # Arguments
/// * `table_name` - Table name where the secrets are stored.
///
/// # Returns
/// * A `Result<Value, String>` object with decrypted payload.
pub fn get_table(table_name: &String) -> Result<Value, String> {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::GetTable.as_str(),
    ]);
    let request = table_request(url, Method::Get, table_name, &env_config);
    let response = request::make_request(request);
    request::decrypt_response(&env_config, &response)
}

/// List all available table names in the server.
///
/// # Returns
/// * A `Vec<String>` with all the table names.
pub fn list_tables() -> Vec<String> {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::ListTables.as_str(),
    ]);
    let request = request::PreparedRequest {
        url,
        method: Method::Get,
        params: HashMap::new(),
        payload: HashMap::new(),
        headers: request::auth_headers(&env_config.apikey),
    };
    let response = request::make_request(request);
    match response {
        Value::Array(array) => {
            let mut table_names: Vec<String> = Vec::new();
            for value in array {
                match value {
                    Value::String(s) => table_names.push(s),
                    Value::Number(n) => table_names.push(n.to_string()),
                    _ => {
                        println!("Unknown value received for table name: {}", value);
                        exit(1)
                    }
                }
            }
            table_names
        }
        _ => {
            println!("Unexpected value returned: {:?}", response);
            exit(1)
        }
    }
}

/// Creates or updates a secret value stored in a table.
///
/// # Arguments
/// * `secrets` - HashMap of secrets as key-value pairs.
/// * `table_name` - Table name where the secret has to be added/updated.
///
/// # Returns
/// * A `Value` object with response from the server.
pub fn put_secret(secrets: &HashMap<String, String>, table_name: &String) -> Value {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::PutSecret.as_str(),
    ]);
    let sec: Value = to_value(secrets.to_owned()).unwrap();
    let mut payload = HashMap::new();
    payload.insert("secrets".to_string(), sec);
    payload.insert("table_name".to_string(), Value::String(table_name.to_string()));
    let request = request::PreparedRequest {
        url,
        method: Method::Put,
        params: HashMap::new(),
        payload,
        headers: request::auth_headers(&env_config.apikey),
    };
    request::make_request(request)
}

/// Deletes a secret stored in a table.
///
/// # Arguments
/// * `key` - Secret key that has to be deleted.
/// * `table_name` - Table name where the secret exists.
///
/// # Returns
/// * A `Value` object with response from the server.
pub fn delete_secret(key: &String, table_name: &String) -> Value {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::DeleteSecret.as_str(),
    ]);
    let request = request::PreparedRequest {
        url,
        method: Method::Delete,
        params: HashMap::new(),
        payload: HashMap::from([
            ("key".to_string(), Value::String(key.to_string())),
            ("table_name".to_string(), Value::String(table_name.to_string())),
        ]),
        headers: request::auth_headers(&env_config.apikey),
    };
    request::make_request(request)
}

/// Creates a new table.
///
/// # Arguments
/// * `table_name` - Table name that has to be created.
///
/// # Returns
/// * A `Value` object with response from the server.
pub fn create_table(table_name: &String) -> Value {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::CreateTable.as_str(),
    ]);
    let request = table_request(url, Method::Post, table_name, &env_config);
    request::make_request(request)
}
