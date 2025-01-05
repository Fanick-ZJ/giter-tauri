

pub fn validate_git_repository(repository: &str) -> Result<gix::Repository, String> {
    let git_repository = gix::open(repository);
    if git_repository.is_err() {
        return Err(git_repository.unwrap_err().to_string());
    }
    let repository = git_repository.ok().unwrap();
    Ok(repository)
}


pub fn has_git() -> bool {
    if let Err(_) = std::process::Command::new("git").arg("--version").output() {
        return false; // git 命令运行失败，说明没有安装 git
    }
    true
}