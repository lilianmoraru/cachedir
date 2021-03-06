use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_app_cache_dir(cache_name:    &Path,
                            app_cache_dir: &Path) -> io::Result<PathBuf> {
        let current_dir = env::current_dir();
        if let Err(err) = current_dir {
            return Err(io::Error::new(err.kind(),
                                      format!("{}\n[Application Cache]: Could not obtain the \
                                              current directory", err.description())));
        }

        let app_cache_dir = current_dir.unwrap().join(app_cache_dir.to_path_buf());
        if let Err(err) = fs::create_dir_all(&app_cache_dir) {
            return Err(io::Error::new(err.kind(),
                                      format!("{}\n[Application Cache]: Failed to create the \
                                              parent cache directory: {}",
                                              err.description(), app_cache_dir.display())));
        }

        super::create_dir_helper(&[app_cache_dir], &cache_name)
    }

    fn create_user_cache_dir(cache_name: &Path)   -> io::Result<PathBuf> {
        let cache_dir = env::home_dir()
                            .and_then(|path| {
                                if path.as_os_str().is_empty() {
                                    None
                                } else {
                                    Some(path.join(".cache"))
                                }
                            });

        if cache_dir.is_none() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                                      "[User Cache]: Could not obtain user's home directory"));
        }

        let cache_dir = cache_dir.unwrap();
        if let Err(err) = fs::create_dir_all(&cache_dir) {
            return Err(io::Error::new(err.kind(), format!("{}\n\
                                                  [User Cache]: Failed to create the parent \
                                                  cache directory", err.description())));
        }

        super::create_dir_helper(&[cache_dir], &cache_name)
    }

    fn create_system_cache_dir(_: &Path)          -> io::Result<PathBuf> {
        // I don't know where the system-wide cache folder is in `Redox`
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[System Cache]: System caches are not supported on Redox"))
    }

    fn create_tmp_cache_dir(cache_name: &Path)    -> io::Result<PathBuf> {
        let temp_dir = env::temp_dir();

        if temp_dir.as_os_str().is_empty() {
            Err(io::Error::new(io::ErrorKind::NotFound,
                               "[Tmp Cache]: Could not obtain the temporary directory's path"))
        } else {
            super::create_dir_helper(&[temp_dir], &cache_name)
        }
    }

    fn create_memory_cache_dir(_: &Path)          -> io::Result<PathBuf> {
        Err(io::Error::new(io::ErrorKind::NotFound,
                           "[Memory Cache]: Memory caches are not supported on Redox"))
    }
}
