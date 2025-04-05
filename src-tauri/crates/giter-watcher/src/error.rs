use giter_traits::ExposeError;
use strum_macros::{EnumDiscriminants, EnumIter};
use thiserror::Error;

#[derive(Error, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum ErrorCode {
  #[error("Add watcher failed: {0}")]
  AddWatcherFailed(String),
  #[error("Remove watcher failed: {0}")]
  RemoveWatcherFailed(String),
  #[error("Other error: {0}")]
  Other(String),
}


impl ExposeError for ErrorCode {
  fn code(&self) -> u32 {
     ErrorCodeDiscriminants::from(self) as u32
  }
  
  fn module(&self) -> &str {
      return "giter-watcher"
  }
}
