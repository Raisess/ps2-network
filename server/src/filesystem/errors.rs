#[derive(Debug)]
pub(super) struct FileNotExistsError {
    pub filename: String,
}

impl std::fmt::Display for FileNotExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File do not exists: {}", self.filename)
    }
}

impl std::error::Error for FileNotExistsError {}

#[derive(Debug)]
pub(super) struct DirNotExistsError {
    pub dirname: String,
}

impl std::fmt::Display for DirNotExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Directory do not exists: {}", self.dirname)
    }
}

impl std::error::Error for DirNotExistsError {}
