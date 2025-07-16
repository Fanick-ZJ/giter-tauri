use std::fmt;

use git2::{Oid, Repository};
use serde::{
    de::{Error as DeError, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::error::GitUtilsErrorCode;

#[derive(Debug, Clone)]
pub struct Commit {
    pub commit_id: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub title: String,
    pub message: String,
    pub datetime: i64,
    pub parents: Vec<Oid>,
    pub repo: String,
}

impl Commit {
    pub fn new(
        commit_id: String,
        author_name: String,
        author_email: String,
        committer_name: String,
        committer_email: String,
        title: String,
        message: String,
        datetime: i64,
        parents: Vec<Oid>,
        repo: String,
    ) -> Self {
        Commit {
            commit_id,
            author_name,
            author_email,
            committer_name,
            committer_email,
            title,
            message,
            datetime,
            parents,
            repo,
        }
    }

    pub fn from_oid(oid: Oid, repo: &Repository) -> Result<Self, GitUtilsErrorCode> {
        let commit = repo.find_commit(oid)?;
        let time = commit.time().seconds();
        let author = commit.author();
        let author_name = String::from_utf8_lossy(author.name_bytes()).to_string();
        let author_email = String::from_utf8_lossy(author.email_bytes()).to_string();
        let committer = commit.committer();
        let comitter_name = String::from_utf8_lossy(committer.name_bytes()).to_string();
        let comitter_email = String::from_utf8_lossy(committer.email_bytes()).to_string();
        let parents = commit.parent_ids().into_iter().collect::<Vec<Oid>>();
        let message = String::from_utf8_lossy(commit.message_bytes()).to_string();
        let title = message.lines().next().unwrap_or("").to_string();
        let commit_id = commit.id().to_string();
        let repo = repo.path().to_str().unwrap().to_string();
        Ok(Commit::new(commit_id, author_name, author_email, comitter_name, comitter_email, title, message, time * 1000, parents, repo))
    }
}

impl Serialize for Commit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Commit", 10)?;
        state.serialize_field("commitId", &self.commit_id)?;
        state.serialize_field("authorName", &self.author_name)?;
        state.serialize_field("authorEmail", &self.author_email)?;
        state.serialize_field("committerName", &self.committer_name)?;
        state.serialize_field("committerEmail", &self.committer_email)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("datetime", &self.datetime)?;
        state.serialize_field(
            "parents",
            &self
                .parents
                .iter()
                .map(|oid| oid.to_string())
                .collect::<Vec<_>>(),
        )?;
        state.serialize_field("repo", &self.repo)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Commit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitVisitor;

        impl<'de> Visitor<'de> for CommitVisitor {
            type Value = Commit;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Commit")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Commit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut commit_id = None;
                let mut author_name = None;
                let mut author_email = None;
                let mut committer_name = None;
                let mut committer_email = None;
                let mut title = None;
                let mut message = None;
                let mut datetime = None;
                let mut parents: Option<Vec<String>> = None;
                let mut repo = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "commitId" => commit_id = Some(map.next_value()?),
                        "authorName" => author_name = Some(map.next_value()?),
                        "authorEmail" => author_email = Some(map.next_value()?),
                        "committerName" => committer_name = Some(map.next_value()?),
                        "committerEmail" => committer_email = Some(map.next_value()?),
                        "title" => title = Some(map.next_value()?),
                        "message" => message = Some(map.next_value()?),
                        "datetime" => datetime = Some(map.next_value()?),
                        "parents" => parents = Some(map.next_value()?),
                        "repo" => repo = Some(map.next_value()?),
                        _ => (),
                    }
                }

                let parents = parents.unwrap_or_default();
                let parents = parents
                    .into_iter()
                    .map(|s| Oid::from_str(&s).map_err(DeError::custom))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Commit {
                    commit_id: commit_id.ok_or_else(|| DeError::missing_field("commitId"))?,
                    author_name: author_name.ok_or_else(|| DeError::missing_field("authorName"))?,
                    author_email: author_email
                        .ok_or_else(|| DeError::missing_field("authorEmail"))?,
                    committer_name: committer_name
                        .ok_or_else(|| DeError::missing_field("committerName"))?,
                    committer_email: committer_email
                        .ok_or_else(|| DeError::missing_field("committerEmail"))?,
                    title: title.ok_or_else(|| DeError::missing_field("title"))?,
                    message: message.ok_or_else(|| DeError::missing_field("message"))?,
                    datetime: datetime.ok_or_else(|| DeError::missing_field("datetime"))?,
                    parents,
                    repo: repo.ok_or_else(|| DeError::missing_field("repo"))?,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "commitId",
            "authorName",
            "authorEmail",
            "committerName",
            "committerEmail",
            "title",
            "message",
            "datetime",
            "parents",
            "repo",
        ];

        deserializer.deserialize_struct("Commit", FIELDS, CommitVisitor)
    }
}