/**
 * Directory - A representation of the filesystem directory
 */
pub(super) struct Directory {
    path: String,
}

impl Directory {
    /**
     * Creates a new `Directory` instance using a path
     *
     * @param String
     */
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /**
     * Returns the directory path
     */
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /**
     * Check if a sub directory exists in the current `Directory`
     *
     * @param &String
     */
    pub fn dir_exists(&self, dirname: &String) -> bool {
        std::path::Path::new(&format!("{}/{dirname}", self.path)).is_dir()
    }

    /**
     * Check if a file exists in the current `Directory`
     *
     * @param &String
     */
    pub fn file_exists(&self, filename: &String) -> bool {
        println!("{}", &format!("{}/{filename}", self.path));
        std::path::Path::new(&format!("{}/{filename}", self.path)).is_file()
    }

    /**
     * Returns the file size in MB in the current `Directory`
     * @NOTE: The file should exists otherwise the program will blow up!
     *
     * @param &String
     */
    pub fn file_size(&self, filename: &String) -> u64 {
        let metadata = std::fs::metadata(&format!("{}/{filename}", self.path)).unwrap();
        metadata.len() / 1_000_000
    }

    /**
     * Moves a file from an source `Directory` to an target `Directory`
     * renaming if desired
     * @NOTE: The file should exists otherwise the program will blow up!
     *
     * @param &Directory
     * @param &String
     * @param &Directory
     * @param &String
     */
    pub fn move_file(
        source: &Self,
        source_filename: &String,
        destination: &Self,
        destination_filename: &String,
    ) -> () {
        let source_full_path = format!("{}/{source_filename}", source.path());
        let destination_full_path = format!("{}/{destination_filename}", destination.path());

        std::fs::rename(source_full_path, destination_full_path).unwrap();
    }
}
