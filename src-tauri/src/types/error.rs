use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug)]
pub enum CommandError {
    RepositoryHasWatched(String),
    InvalidRepository(String),
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
            }
            CommandError::InvalidRepository(ref path) => {
                s.serialize_field("error", "InvalidRepository")?;
                s.serialize_field("data", path)?;
            }
        }
        s.end()
    }
}
