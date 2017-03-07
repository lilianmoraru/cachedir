use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_user_cache_dir(dir_name: &Path)   -> io::Result<PathBuf> {
        super::create_dir_helper(&[], &dir_name)
    }

    fn create_system_cache_dir(dir_name: &Path) -> io::Result<PathBuf> {
        super::create_dir_helper(&[], &dir_name)
    }

    fn create_tmp_cache_dir(dir_name: &Path)    -> io::Result<PathBuf> {
        super::create_dir_helper(&[], &dir_name)
    }
}
