#[derive(Debug)]
pub struct Commit {
    pub commit_id: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub title: String,
    pub message: String,
    pub datetime: i64,
    pub parents_count: i64,
    pub repo: String,
}

impl Commit {
    pub fn new(commit_id: String,
               author_name: String,
               author_email: String,
               committer_name: String,
               committer_email: String,
               title: String,
               message: String,
               datetime: i64,
               parents_count: i64,
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
            parents_count,
            repo
        }
    }
}