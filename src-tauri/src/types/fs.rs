#[derive(Debug, Clone)]
pub struct File {
  pub name: String,
  pub path: String,
  pub size: u64,
}
impl File {
  pub fn new(name: String, path: String, size: u64) -> File {
    File {
      name,
      path,
      size,
    }
  }
}
#[derive(Debug, Clone)]
pub struct Dir {
  pub name: String,
  pub path: String,
  pub is_repo: bool,
}

impl Dir {
  pub fn new(name: String, path: String, is_repo: bool) -> Dir {
    Dir {
      name,
      path,
      is_repo,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Catalog {
  pub dirs: Vec<Dir>,
  pub files: Vec<File>,
}
impl Catalog {
  pub fn new(dirs: Vec<Dir>, files: Vec<File>) -> Catalog {
    Catalog {
      dirs,
      files,
    }
  }
}