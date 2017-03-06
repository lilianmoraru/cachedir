use std::path::{ Path, PathBuf };
use std::io::{ self, Error, ErrorKind };

use super::{ CacheDirImpl, CacheDirOperations, create_dir };

impl CacheDirOperations for CacheDirImpl {
    fn create_user_cache_dir(dir_name: &Path)   -> io::Result<PathBuf> {
        create_dir(&[])
    }

    fn create_system_cache_dir(dir_name: &Path) -> io::Result<PathBuf> {
        create_dir(&[])
    }

    fn create_tmp_cache_dir(dir_name: &Path)    -> io::Result<PathBuf> {
        create_dir(&[])
    }
}
