use crate::{enums, parser, request, util};
use serde_json::Value;
use std::collections::HashMap;
use std::process::exit;

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
    let request = request::RequestMaterials {
        url,
        params,
        headers: request::auth_headers(&env_config.apikey),
    };
    let response = request::make_request(&request.url, Some(request.headers), Some(request.params));
    request::decrypt_response(&env_config, &response)
}

pub fn get_table(table: &String) -> Result<Value, String> {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::GetTable.as_str(),
    ]);
    let params = HashMap::from([("table_name".to_string(), table.to_string())]);
    let request = request::RequestMaterials {
        url,
        params,
        headers: request::auth_headers(&env_config.apikey),
    };
    let response = request::make_request(&request.url, Some(request.headers), Some(request.params));
    request::decrypt_response(&env_config, &response)
}

pub fn list_tables() -> Vec<String> {
    let env_config = parser::env_variables();
    let url = util::urljoin(&[
        env_config.vault_server.as_ref(),
        enums::EndpointMapping::ListTables.as_str(),
    ]);
    let request = request::RequestMaterials {
        url,
        params: HashMap::new(),
        headers: request::auth_headers(&env_config.apikey),
    };
    let response = request::make_request(&request.url, Some(request.headers), Some(request.params));
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
