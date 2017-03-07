use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_user_cache_dir(cache_name: &Path,
                             parent_dir: Option<&Path>)   -> io::Result<PathBuf> {
        super::create_dir_helper(&[], &cache_name)
    }

    fn create_system_cache_dir(cache_name: &Path) -> io::Result<PathBuf> {
        super::create_dir_helper(&[], &cache_name)
    }

    fn create_tmp_cache_dir(cache_name: &Path)    -> io::Result<PathBuf> {
        super::create_dir_helper(&[], &cache_name)
    }

    fn create_memory_cache_dir(_: &Path)          -> io::Result<PathBuf> {
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[Memory Cache]: Memory caches are not supported on Windows"))
    }
}
