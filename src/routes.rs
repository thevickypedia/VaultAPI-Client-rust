use crate::enums::Method;
use crate::parser::EnvConfig;
use crate::{enums, parser, request, util};
use serde_json::{to_value, Value};
use std::collections::HashMap;
use std::process::exit;

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

pub fn get_secret(key: &String, table: &String) -> Result<Value, String> {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::GetSecret.as_str(),
    ]);
    let params = HashMap::from([
        ("table_name".to_string(), table.to_string()),
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

pub fn get_table(table: &String) -> Result<Value, String> {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::GetTable.as_str(),
    ]);
    let request = table_request(url, Method::Get, table, &env_config);
    let response = request::make_request(request);
    request::decrypt_response(&env_config, &response)
}

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

pub fn create_table(table_name: &String) -> Value {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::CreateTable.as_str(),
    ]);
    let request = table_request(url, Method::Post, table_name, &env_config);
    request::make_request(request)
}
