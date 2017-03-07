use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_user_cache_dir(dir_name: &Path)   -> io::Result<PathBuf> {
        // Lets see if we can get the `$HOME` path - it could be missing
        // in a bare-bones `Linux` container or in `Emscripten` - as examples
        let cache_dir: PathBuf;
        let home_dir = env::home_dir();

        if let None = home_dir {
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
            cache_dir = { home_dir.unwrap() }.join(".cache");
        }

        // Lets make sure that the parent cache directory exists
        if let Err(err) = fs::create_dir_all(&cache_dir) {
            return Err(io::Error::new(err.kind(), format!("{}\n\
                                                  [User Cache]: Failed to create the parent \
                                                  cache directory", err.description())));
        }

        super::create_dir_helper(&[cache_dir], &dir_name)
    }

    fn create_system_cache_dir(dir_name: &Path) -> io::Result<PathBuf> {
        super::create_dir_helper(&[PathBuf::from("/var/cache")], &dir_name)
    }

    fn create_tmp_cache_dir(dir_name: &Path)    -> io::Result<PathBuf> {
        // We try `/dev/tmp` first because the directory is persistent between system restarts
        super::create_dir_helper(&[PathBuf::from("/var/tmp"), env::temp_dir()],
                                 &dir_name)
    }
}
