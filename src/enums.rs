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
