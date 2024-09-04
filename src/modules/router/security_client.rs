pub enum SecurityClientId {
    Id(String),
    Anonymous,
}

impl ToString for SecurityClientId {
    fn to_string(&self) -> String {
        match self {
            SecurityClientId::Id(id) => id.clone(),
            SecurityClientId::Anonymous => "anonymous-client".to_string(),
        }
    }
}

impl From<SecurityClientId> for String {
    fn from(security_client_id: SecurityClientId) -> Self {
        match security_client_id {
            SecurityClientId::Id(id) => id,
            SecurityClientId::Anonymous => "anonymous-client".to_string(),
        }
    }
}
