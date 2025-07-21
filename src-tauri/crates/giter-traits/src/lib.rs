pub trait ExposeError {
    fn code(&self) -> u32;
    fn module(&self) -> &str;
}

impl std::fmt::Display for dyn ExposeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.module())
    }
}
