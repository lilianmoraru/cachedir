use std::error::Error;
use std::path::{ Path, PathBuf };
use std::io;
use std::fs;
use std::env;

use super::{ CacheDirImpl, CacheDirOperations };

impl CacheDirOperations for CacheDirImpl {
    fn create_app_cache_dir(cache_name:    &Path,
                            app_cache_dir: &Path) -> io::Result<PathBuf> {
        let app_cache_dir = PathBuf::from(app_cache_dir);
        if let Err(err) = fs::create_dir_all(&app_cache_dir) {
            return Err(io::Error::new(err.kind(),
                                      format!("{}\n[Application Cache]: Failed to create the \
                                              parent cache directory: {}",
                                              err.description(), app_cache_dir.display())));
        }

        super::create_dir_helper(&[app_cache_dir], &cache_name)
    }

    fn create_user_cache_dir(cache_name: &Path)   -> io::Result<PathBuf> {
        // We try(and fallback to the next if it fails):
        // 1. Windows environment variable: %LOCALAPPDATA%
        // 2. %APPDATA%
        // 3. Rust's `home_dir` which returns %HOME% -ifndef-> %USERPROFILE% -ifndef-> OS syscall.
        //    Currently `home_dir` has a bug and can return an empty string, so we check that too.
        //    Since the user's home directory is not a dedicated cache dir, we try to add the "Cache" dir
        let mut errors_buffer = String::new();
        let mut cache_dirs: Vec<PathBuf> = Vec::with_capacity(3);

        fn add_env_path(errors_buffer: &mut String,
                        cache_dirs:    &mut Vec<PathBuf>,
                        env_var:       &str) {
            env::var_os(env_var)
                .or_else(|| {
                    errors_buffer.push_str(&format!("\n[User Cache]: %{}% is undefined", env_var));
                    None
                })
                .map(|path| {
                    if !path.is_empty() {
                        cache_dirs.push(PathBuf::from(path))
                    } else {
                        errors_buffer.push_str(&format!("\n[User Cache]: %{}% is defined, \
                                                        but it is set to an empty string", env_var))
                    }
                });
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
            .map(|path| {
                if let Err(err) = fs::create_dir_all(&path) {
                    errors_buffer.push_str(
                        &format!("\n{}\n[User Cache][{:?}]: \
                                 Failed to create the parent cache directory: {}",
                                 err.description(), err.kind(), path.display()));
                } else {
                    cache_dirs.push(path);
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
    }

    fn create_system_cache_dir(cache_name: &Path) -> io::Result<PathBuf> {
        use std::mem;

        // Initialized and used only when `program_data` is `None`
        let mut error: String = unsafe { mem::uninitialized() };
        let program_data = env::var_os("ProgramData")
                               .or_else(|| {
                                   error = String::from("%ProgramData% is not set");
                                   None
                               })
                               .and_then(|path| {
                                   if path.as_os_str().is_empty() {
                                       error = String::from("%ProgramData% is set but it is empty");
                                       None
                                   } else {
                                       Some(path)
                                   }
                               });

        if program_data.is_none() {
            Err(io::Error::new(io::ErrorKind::NotFound,
                               format!("[System Cache]: Could not obtain the path to the cache \
                                        because {}", error)))
        } else {
            let program_data = PathBuf::from(program_data.unwrap());
            super::create_dir_helper(&[program_data], &cache_name)
        }
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
                           "[Memory Cache]: Memory caches are not supported on Windows"))
    }
}
