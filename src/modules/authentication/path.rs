pub enum AuthenticationPath {
    StrPath(String),
    TwoFactorAuthorize,
    TwoFactorRecover,
    TwoFactorVerify,
}

impl std::fmt::Display for AuthenticationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationPath::StrPath(path) => write!(f, "{path}"),
            AuthenticationPath::TwoFactorAuthorize => write!(f, "/authentication/twoFa/authorize"),
            AuthenticationPath::TwoFactorRecover => write!(f, "/authentication/twoFa/authorize"),
            AuthenticationPath::TwoFactorVerify => write!(f, "/authentication/twoFa/verify",),
        }
    }
}

impl From<AuthenticationPath> for String {
    fn from(path: AuthenticationPath) -> Self {
        match path {
            AuthenticationPath::StrPath(path) => path,
            AuthenticationPath::TwoFactorAuthorize => "/authentication/twoFa/authorize".to_string(),
            AuthenticationPath::TwoFactorRecover => "/authentication/twoFa/authorize".to_string(),
            AuthenticationPath::TwoFactorVerify => "/authentication/twoFa/verify".to_string(),
        }
    }
}
