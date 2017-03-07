use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_user_cache_dir(dir_name: &Path)   -> io::Result<PathBuf> {
        let home_dir = env::home_dir();
        if let None = home_dir {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                                      "[User Cache]: Could not obtain user's home directory"));
        }

        // Lets make sure that the parent cache directory exists
        let cache_dir = { home_dir.unwrap() }.join(".cache");
        if let Err(err) = fs::create_dir_all(&cache_dir) {
            return Err(io::Error::new(err.kind(), format!("{}\n\
                                                  [User Cache]: Failed to create the parent \
                                                  cache directory", err.description())));
        }

        super::create_dir_helper(&[cache_dir], &dir_name)
    }

    fn create_system_cache_dir(_: &Path) -> io::Result<PathBuf> {
        // I don't know where the system-wide cache folder is in `Redox`
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[System Cache]: This OS is not supported"))
    }

    fn create_tmp_cache_dir(dir_name: &Path)    -> io::Result<PathBuf> {
        super::create_dir_helper(&[env::temp_dir()], &dir_name)
    }
}
