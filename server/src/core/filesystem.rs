pub mod directory;

use directory::Directory;

pub enum FilesystemError {
    FileNotExistsError(String),
    DirNotExistsError(String),
}

const MAXIMUM_CD_SIZE_MB: u64 = 700;

/**
 * Filesystem - A representation of the filesystem
 *
 * Can manage files and folders inside a source and a target `Directory`.
 */
pub struct Filesystem {
    source_directory: Directory,
    taget_directory: Directory,
}

impl Filesystem {
    /**
     * Creates a new `Filesystem` instance for a source and a target `Directory`
     *
     * @param String
     * @param String
     */
    pub fn new(source_directory_path: String, taget_directory_path: String) -> Self {
        Self {
            source_directory: Directory::new(source_directory_path),
            taget_directory: Directory::new(taget_directory_path),
        }
    }

    /**
     * Move a game from the source to the target aiming the correct subdir (DVD, CD)
     *
     * @param String - "SCUS_973.99.God of War.iso"
     */
    pub fn move_game(&self, filename: String) -> Result<(), FilesystemError> {
        if !self.source_directory.file_exists(&filename) {
            return Err(FilesystemError::FileNotExistsError(filename));
        }

        let dvd_dir = "DVD".to_string();
        if !self.taget_directory.dir_exists(&dvd_dir) {
            return Err(FilesystemError::DirNotExistsError(dvd_dir));
        }

        let cd_dir = "CD".to_string();
        if !self.taget_directory.dir_exists(&cd_dir) {
            return Err(FilesystemError::DirNotExistsError(cd_dir));
        }

        let target_dir = if self.source_directory.file_size(&filename) > MAXIMUM_CD_SIZE_MB {
            "DVD"
        } else {
            "CD"
        };

        let target_filename = format!("{target_dir}/{filename}");
        Ok(Directory::move_file(
            &self.source_directory,
            &filename,
            &self.taget_directory,
            &target_filename,
        ))
    }
}
