/// Enum to load the endpoint mapping.
#[derive(Debug)]
pub enum EndpointMapping {
    Health,
    GetTable,
    GetSecret,
    PutSecret,
    ListTables,
    CreateTable,
    DeleteSecret,
}

/// Implements the endpoint mapping enum to the appropriate API endpoints.
impl EndpointMapping {
    pub fn as_str(&self) -> &str {
        match self {
            EndpointMapping::Health => "/health",
            EndpointMapping::GetTable => "/get-table",
            EndpointMapping::GetSecret => "/get-secret",
            EndpointMapping::PutSecret => "/put-secret",
            EndpointMapping::ListTables => "/list-tables",
            EndpointMapping::CreateTable => "/create-table",
            EndpointMapping::DeleteSecret => "/delete-secret",
        }
    }
}

/// Enum to load the API methods.
#[derive(Debug)]
pub enum Method {
    Get,
    Put,
    Delete,
    Post,
}

/// Implements the match object to validate as conditions.
impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other),
            (Method::Get, Method::Get) |
            (Method::Post, Method::Post) |
            (Method::Put, Method::Put) |
            (Method::Delete, Method::Delete)
        )
    }
}
