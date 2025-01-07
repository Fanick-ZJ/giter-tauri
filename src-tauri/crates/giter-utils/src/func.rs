

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

#[cfg(test)]
mod tests {
    use std::path::Path;
    use gix::discover::repository::Kind;
    use func::validate_git_repository;

    #[test]
    fn is_git() {
        let dir_path = Path::new(r"E:\workSpace\Rust\GQL\.git");
        let ret = gix::discover::is_git(dir_path);
        match ret {
            Ok(kind) => {
                match &kind {
                    Kind::PossiblyBare => {
                        println!("is bare");
                    }
                    Kind::WorkTree { .. } => {
                        println!("is worktree");
                    }
                    Kind::WorkTreeGitDir { .. } => {
                        println!("is work tree");
                    }
                    Kind::Submodule { .. } => {
                        println!("is submodule");
                    }
                    Kind::SubmoduleGitDir => {
                        println!("is submodule git");
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    ///此函数用来测试一个路径/父级是否为仓库
    ///
    #[test]
    fn upwards() {
        let dir_path = Path::new(r"E:\workSpace\Rust");
        let ret = gix::discover::upwards(dir_path);
        println!("{:?}", ret);
    }

    #[test]
    fn open() {
        let dir_path = Path::new(r"E:\workSpace\Rust\GQL");
        let ret = gix::open(dir_path);
        println!("{:?}", ret);
    }

    #[test]
    fn test_validate_git_repository() {
        let ret = validate_git_repository(r"E:\workSpace\Rust\GQL");
        match &ret {
            Ok(_) => {
                println!("validate git repository");
            }
            Err(_) => {
                println!("validate git repository error");
            }
        }
    }
}