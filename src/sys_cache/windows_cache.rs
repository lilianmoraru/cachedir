use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_app_cache_dir(cache_name:    &Path,
                            app_cache_dir: &Path) -> io::Result<PathBuf> {
        unimplemented!()
    }

    fn create_user_cache_dir(cache_name: &Path)   -> io::Result<PathBuf> {
        unimplemented!();
        /*
        // We try(and fallback to the next if it fails):
        // 1. User-passed `parent_dir`
        // 2. Windows environment variable: %LOCALAPPDATA%
        // 3. %APPDATA%
        // 4. Rust's `home_dir` which returns %HOME% -ifndef-> %USERPROFILE% -ifndef-> OS syscall.
        //    Currently `home_dir` has a bug and can return an empty string, so we check that too.
        //    Since the user's home directory is not a dedicated cache dir, we try to add the "Cache" dir
        if parent_dir.is_some() {
            if let Err(err) = fs::create_dir_all(&parent_dir.unwrap()) {
                return Err(io::Error::new(err.kind(), format!("{}\n\
                                                  [User Cache]: Failed to create the parent \
                                                  cache directory", err.description())));
            }

            return super::create_dir_helper(&[parent_dir.unwrap().to_path_buf()],
                                            &cache_name);
        }

        let mut errors_buffer = String::new();
        let mut cache_dirs: Vec<PathBuf> = vec![];

        fn add_env_path(errors_buffer: &mut String,
                        cache_dirs:    &mut Vec<PathBuf>,
                        env_var:       &str) {
            env::var_os(env_var)
                .or_else(|| {
                    errors_buffer.push_str(&format!("\n[User Cache]: %{}% is undefined", env_var));
                    None
                })
                .map(|path| if !path.is_empty() {
                                cache_dirs.push(PathBuf::from(path))
                            } else {
                                errors_buffer.push_str(&format!("\n[User Cache]: %{}% is defined, \
                                                        but it is set to an empty string", env_var))
                            }
                );
        }

        add_env_path(&mut errors_buffer, &mut cache_dirs, "LOCALAPPDATA");
        add_env_path(&mut errors_buffer, &mut cache_dirs, "APPDATA");

        env::home_dir()
            .or_else(|| {
                errors_buffer.push_str(
                    &format!("\n[User Cache]: Could not obtain user's home directory"));
                None
            })
            .and_then(|path| {
                if !path.as_os_str().is_empty() {
                    Some(path.join("Cache"))
                } else {
                    errors_buffer.push_str(
                        &format!("\n[User Cache]: %HOME% and/or %USERPROFILE% variables \
                        are/is defined but are/is set to an empty string"));
                    None
                }
            })
            .and_then(|path| {
                if let Err(err) = fs::create_dir_all(&path) {
                    errors_buffer.push_str(
                        &format!("\n{}\n[User Cache][{:?}]: \
                                 Failed to create the parent cache directory: {}",
                                 err.description(), err.kind(), path.display()));
                    None
                } else {
                    cache_dirs.push(path);
                    Some(..)
                }
            });

        if cache_dirs.is_empty() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("{}\n[User Cache]: Could not obtain user's cache directory",
                            errors_buffer)
                )
            );
        }

        match super::create_dir_helper(&cache_dirs, &cache_name) {
            Ok(result) => Ok(result),
            Err(err)   => Err(io::Error::new(err.kind(),
                                             format!("{}\n{}", errors_buffer, err.description())))
        }
        */
    }

    fn create_system_cache_dir(cache_name: &Path) -> io::Result<PathBuf> {
        //ProgramData
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
