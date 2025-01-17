use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug)]
pub enum CommandError {
    RepositoryHasWatched(String),
    AddWatcherError(String),
    AddRepositoryStoreError(String),
    InvalidRepository(String),
    FindAuthorsError(String),
    DataProviderBuildError(String),
    BranchNotFound(String),
    BranchesFindError(String),
    GetAuthorError(String),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CommandError", 2)?;
        match *self {
            CommandError::RepositoryHasWatched(ref path) => {
                s.serialize_field("error", "RepositoryHasWatched")?;
                s.serialize_field("data", path)?;
            },
            CommandError::AddWatcherError(ref path) => {
                s.serialize_field("error", "AddWatcherError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::AddRepositoryStoreError(ref path) => {
                s.serialize_field("error", "AddRepositoryStoreError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::InvalidRepository(ref path) => {
                s.serialize_field("error", "InvalidRepository")?;
                s.serialize_field("data", path)?;
            },
            CommandError::FindAuthorsError(ref path) => {
                s.serialize_field("error", "FindAuthorsError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::BranchNotFound(ref path) => {
                s.serialize_field("error", "BranchNotFound")?;
                s.serialize_field("data", path)?;
            },
            CommandError::DataProviderBuildError(ref path) => {
                s.serialize_field("error", "DataProviderBuildError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::GetAuthorError(ref path) => {
                s.serialize_field("error", "GetAuthorError")?;
                s.serialize_field("data", path)?;
            },
            CommandError::BranchesFindError(ref path) => {
                s.serialize_field("error", "BranchesFindError")?;
                s.serialize_field("data", path)?;
            },
        }
        s.end()
    }
}
