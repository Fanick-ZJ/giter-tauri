#[derive(Debug, Clone)]
pub enum Credential {
    Token(String),
    UsernamePassword(String, String),
}