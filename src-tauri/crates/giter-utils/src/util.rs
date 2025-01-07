use gix;
use crate::types::commit::Commit;

pub fn build_commit(commit: &gix::Commit) -> Commit {
  Commit::new (
    commit.id.to_string(),
    commit.author().unwrap().name.to_string(),
    commit.author().unwrap().email.to_string(),
    commit.committer().unwrap().name.to_string(),
    commit.committer().unwrap().email.to_string(),
    commit.message().unwrap().summary().to_string(),
    commit.message().unwrap().body().unwrap().to_string(),
    commit.time().unwrap().seconds as i64,
    commit.parent_ids().into_iter().count() as i64,
    commit.repo.path().to_str().unwrap().to_string(),
  )
}
