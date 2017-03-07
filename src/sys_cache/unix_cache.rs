use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_user_cache_dir(cache_name: &Path,
                             parent_dir: Option<&Path>)   -> io::Result<PathBuf> {
        // Lets see if we can get the `$HOME` path - it could be missing
        // in a bare-bones `Linux` container or in `Emscripten` - as examples
        let cache_dir: PathBuf;
        let home_dir = if parent_dir.is_none() {
            env::home_dir()
        } else {
            Some(PathBuf::from(&parent_dir.unwrap()))
        };

        if home_dir.is_none() {
            // On `Emscripten` we have rights to create files and directories anywhere,
            // so we'll still try to create the cache directory without having `$HOME`
            if cfg!(not(target_os = "emscripten")) {
                return Err(io::Error::new(io::ErrorKind::NotFound,
                                          "[User Cache]: Could not obtain user's home directory"));
            } else {
                { home_dir }; // Nobody should use `home_dir` from this point
                cache_dir = PathBuf::from("/var/cache");
            }
        } else {
            // We choose between `$HOME` or the user-passed path stored in `parent_dir`.
            // If we use `$HOME`, then we want to create the cache in:
            // - `$HOME/.cache` on all Unix systems except `macOS`
            // - `$HOME/Lirary/Caches` on `macOS`
            cache_dir = if parent_dir.is_none() {
                if cfg!(not(target_os = "macos")) {
                    { home_dir.unwrap() }.join(".cache")
                } else {
                    { home_dir.unwrap() }.join("Library/Caches")
                }
            } else {
                { home_dir.unwrap() }
            };
        }

        // Lets make sure that the parent cache directory exists
        if let Err(err) = fs::create_dir_all(&cache_dir) {
            return Err(io::Error::new(err.kind(), format!("{}\n\
                                                  [User Cache]: Failed to create the parent \
                                                  cache directory", err.description())));
        }

        super::create_dir_helper(&[cache_dir], &cache_name)
    }

    fn create_system_cache_dir(cache_name: &Path) -> io::Result<PathBuf> {
        if cfg!(not(target_os = "macos")) {
            super::create_dir_helper(&[PathBuf::from("/var/cache")],
                                     &cache_name)
        } else {
            super::create_dir_helper(&[PathBuf::from("/Library/Caches")],
                                     &cache_name)
        }
    }

    fn create_tmp_cache_dir(cache_name: &Path)    -> io::Result<PathBuf> {
        // We try `/var/tmp` first because the directory is persistent between system restarts
        super::create_dir_helper(&[PathBuf::from("/var/tmp"), env::temp_dir()],
                                 &cache_name)
    }

    fn create_memory_cache_dir(cache_name: &Path) -> io::Result<PathBuf> {
        if cfg!(target_os = "linux") {
            super::create_dir_helper(&[PathBuf::from("/dev/shm"), PathBuf::from("/run/shm")],
                                     &cache_name)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound,
                               "[Memory Cache]: Memory caches are not supported on this OS"))
        }
    }
}
