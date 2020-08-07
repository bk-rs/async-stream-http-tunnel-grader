// ref https://github.com/swift-server/async-http-client/blob/1.1.1/Sources/AsyncHTTPClient/HTTPHandler.swift#L248-L280
pub struct Authorization(Inner);
enum Inner {
    Basic(String),
    Bearer(String),
}
impl Authorization {
    pub fn basic(username: String, password: String) -> Self {
        Self(Inner::Basic(base64::encode(format!(
            "{}:{}",
            username, password
        ))))
    }

    pub fn basic_credentials(credentials: String) -> Self {
        Self(Inner::Basic(credentials))
    }

    pub fn bearer(tokens: String) -> Self {
        Self(Inner::Bearer(tokens))
    }

    pub fn header_value(&self) -> String {
        match &self.0 {
            Inner::Basic(value) => format!("Basic {}", value),
            Inner::Bearer(value) => format!("Bearer {}", value),
        }
    }
}
