use std::process::Command;

use serde::{ser::SerializeStruct, Serialize};
use giter_traits::ExposeError;
use strum_macros::{EnumDiscriminants, EnumIter};

use thiserror::Error;


#[derive(Debug)]
pub struct CommandError<T>{
    pub func: String,
    pub error: Option<T>,
}
impl <T> CommandError<T>
where T: std::fmt::Display + giter_traits::ExposeError
{
    pub fn new(func: &str, e: T) -> Self {
        Self {
            func: func.to_string(),
            error: Some(e),
        }
    }
}

impl <T> CommandError<T>
where T: AsRef<dyn giter_traits::ExposeError>
{
    pub fn from_command(func: &str, e: T) -> Self {
        Self {
            func: func.to_string(),
            error: Some(e.into()),
        }
    }
}

impl <T> Serialize for CommandError<T>
where T: std::fmt::Display + giter_traits::ExposeError
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CommandError", 4)?;
        state.serialize_field("code", &self.error.as_ref().unwrap().code())?;
        state.serialize_field("message", &self.error.as_ref().unwrap().to_string())?;
        state.serialize_field("func", &self.func)?;
        state.serialize_field("module", &self.error.as_ref().unwrap().module())?;
        state.end()
    }
}


#[derive(Error, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum ErrorCode {
    #[error("Get watcher center failed")]
    GetWatcherCenterFailed,
    #[error("Get repos failed")]
    GetReposFailed,
    #[error("Database is invalid: {0}")]
    DatabaseInvalid(String),
    #[error("Path is invalid: {0}")]
    PathInvalid(String),
    #[error("Set global config error: {0}")]
    SetGlobalConfigError(String),
    #[error("Get global config error: {0}")]
    GetGlobalConfigError(String),
}

impl ExposeError for ErrorCode {
    fn code(&self) -> u32 {
        ErrorCodeDiscriminants::from(self) as u32
    }
    
    fn module(&self) -> &str {
        return "giter"
    }
}
