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
        // Lets see if we can get the `$HOME` path - it could be missing
        // in a bare-bones `Linux` container or in `Emscripten` - as examples
        let cache_dir = env::home_dir()
                            .and_then(|path| {
                                if path.as_os_str().is_empty() {
                                    None
                                } else {
                                    if cfg!(not(target_os = "macos")) {
                                        Some(path.join(".cache"))
                                    } else {
                                        Some(path.join("Library/Caches"))
                                    }
                                }
                            })
                            .or_else(|| {
                                // On `Emscripten` we have rights to create files
                                // and directories anywhere, so we'll still try to create
                                // the cache directory without having `$HOME`
                                if cfg!(target_os = "emscripten") {
                                    Some(PathBuf::from("/var/cache"))
                                } else {
                                    None
                                }
                            });

        if cache_dir.is_none() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                                      "[User Cache]: Could not obtain user's home directory"));
        }

        // Lets make sure that the parent cache directory exists
        let cache_dir = cache_dir.unwrap();
        if let Err(err) = fs::create_dir_all(&cache_dir) {
            return Err(io::Error::new(err.kind(),
                                      format!("{}\n[User Cache]: Failed to create the \
                                              parent cache directory: {}",
                                              err.description(), cache_dir.display())));
        }

        super::create_dir_helper(&[cache_dir], &cache_name)
    }

    fn create_system_cache_dir(cache_name: &Path) -> io::Result<PathBuf> {
        if cfg!(not(target_os = "macos")) {
            if cfg!(target_os = "emscripten") {
                let _ = fs::create_dir_all("/var/cache");
            }

            super::create_dir_helper(&[PathBuf::from("/var/cache")],
                                     &cache_name)
        } else {
            super::create_dir_helper(&[PathBuf::from("/Library/Caches")],
                                     &cache_name)
        }
    }

    fn create_tmp_cache_dir(cache_name: &Path)    -> io::Result<PathBuf> {
        let temp_dir = env::temp_dir();

        // We try `/var/tmp` first because the directory is persistent between system restarts
        if temp_dir.as_os_str().is_empty() {
            if cfg!(target_os = "emscripten") {
                let _ = fs::create_dir_all("/var/tmp");
            }

            super::create_dir_helper(&[PathBuf::from("/var/tmp")],
                                     &cache_name)
        } else {
            super::create_dir_helper(&[PathBuf::from("/var/tmp"), temp_dir],
                                     &cache_name)
        }
    }

    fn create_memory_cache_dir(cache_name: &Path) -> io::Result<PathBuf> {
        if cfg!(any(target_os = "linux", target_os = "emscripten")) {
            if cfg!(target_os = "emscripten") {
                let _ = fs::create_dir_all("/dev/shm");
            }

            super::create_dir_helper(&[PathBuf::from("/dev/shm"), PathBuf::from("/run/shm")],
                                     &cache_name)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound,
                               "[Memory Cache]: Memory caches are not supported on this OS"))
        }
    }
}
