use std::path::{ Path, PathBuf };
use std::io;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_app_cache_dir(_: &Path, _: &Path) -> io::Result<PathBuf> {
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[Application Cache]: This OS is not supported"))
    }

    fn create_user_cache_dir(_: &Path)   -> io::Result<PathBuf> {
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[User Cache]: This OS is not supported"))
    }

    fn create_system_cache_dir(_: &Path) -> io::Result<PathBuf> {
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[System Cache]: This OS is not supported"))
    }

    fn create_tmp_cache_dir(_: &Path)    -> io::Result<PathBuf> {
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[Tmp Cache]: This OS is not supported"))
    }

    fn create_memory_cache_dir(_: &Path) -> io::Result<PathBuf> {
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[Memory Cache]: This OS is not supported"))
    }
}
