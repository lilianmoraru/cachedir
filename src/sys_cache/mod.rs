use std::io;
use std::path;
use std::fs;

pub fn create_cache_dir(cache_config: &super::CacheDirConfig)
    -> io::Result<path::PathBuf>
{
    use std::error::Error;
    let default_config = !cache_config.user_cache
                         && !cache_config.system_cache
                         && !cache_config.tmp_cache
                         && !cache_config.memory_cache;

    let user_cache = if default_config { true } else { cache_config.user_cache };

    let mut last_io_error = io::ErrorKind::NotFound;
    let mut errors_buffer = String::new();

    if user_cache {
        match CacheDirImpl::create_user_cache_dir(&cache_config.cache_name,
                                                  cache_config.user_parent_dir) {
            Ok(result) => return Ok(result),
            Err(err)   => {
                last_io_error = err.kind();
                errors_buffer.push_str(err.description());
            }
        }
    }

    if cache_config.system_cache {
        match CacheDirImpl::create_system_cache_dir(&cache_config.cache_name) {
            Ok(result) => return Ok(result),
            Err(err)   => {
                last_io_error = err.kind();
                errors_buffer.push_str(err.description());
            }
        }
    }

    if cache_config.tmp_cache {
        match CacheDirImpl::create_tmp_cache_dir(&cache_config.cache_name) {
            Ok(result) => return Ok(result),
            Err(err)   => {
                last_io_error = err.kind();
                errors_buffer.push_str(err.description());
            }
        }
    }

    if cache_config.memory_cache {
        match CacheDirImpl::create_memory_cache_dir(&cache_config.cache_name) {
            Ok(result) => return Ok(result),
            Err(err)   => {
                last_io_error = err.kind();
                errors_buffer.push_str(err.description());
            }
        }
    }

    Err(io::Error::new(last_io_error, errors_buffer))
}

// ===== Private =====
#[cfg(unix)]
#[path = "unix_cache.rs"]
mod cache_impl;

#[cfg(windows)]
#[path = "windows_cache.rs"]
mod cache_impl;

#[cfg(target_os = "redox")]
#[path = "redox_cache.rs"]
mod cache_impl;

#[cfg(not(any(unix, windows, target_os = "redox")))]
#[path = "unknown_os_cache.rs"]
mod cache_impl;

// All os-specific modules implement `CacheDirOperations` on the `CacheDirImpl` structure
struct CacheDirImpl;

// The functions that should be implemented by all os-specific modules
trait CacheDirOperations {
    fn create_user_cache_dir(cache_name: &path::Path,
                             parent_dir: Option<&path::Path>) -> io::Result<path::PathBuf>;
    fn create_system_cache_dir(cache_name: &path::Path) -> io::Result<path::PathBuf>;
    fn create_tmp_cache_dir(cache_name: &path::Path)    -> io::Result<path::PathBuf>;
    fn create_memory_cache_dir(cache_name: &path::Path) -> io::Result<path::PathBuf>;
}

// Common function shared between all implementations of the CacheDirOperations trait
fn create_dir_helper(dirs: &[path::PathBuf], path: &path::Path) -> io::Result<path::PathBuf> {
    // Sadly, we don't have something like `static_assert`
    debug_assert!(!dirs.is_empty(),
                  "Code-logic error: the slice of directories should not be empty");
    if dirs.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound,
                                  "Could not create the cache directory"));
    }

    // This is a buffer of all the errors(attempts) to create a cache directory.
    // The `PermissionDenied` error will be returned with this buffer only if all the attempts fail
    let mut attempted_paths_error = String::new();
    for parent_cache_dir in dirs {
        if !parent_cache_dir.exists() {
            attempted_paths_error.push_str(
                &format!("\n[NotFound]: Parent cache directory does not exist: {}",
                         parent_cache_dir.display()));
        } else {
            if !parent_cache_dir.is_dir() {
                attempted_paths_error.push_str(
                    &format!("\n[AlreadyExists]: Parent cache path is not a directory: {}",
                             parent_cache_dir.display())
                );
            } else {
                let final_cache_path = &parent_cache_dir.join(path);
                if let Err(err) = fs::create_dir_all(&final_cache_path) {
                    attempted_paths_error.push_str(
                        &format!("\n[{:?}]: Failed to create the cache directory: {}",
                                 err.kind(),
                                 final_cache_path.display()));
                } else {
                    return Ok(path::PathBuf::from(final_cache_path));
                }
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::PermissionDenied,
                       attempted_paths_error))
}

// Making sure that the `CacheDirOperations` trait was implemented on `CacheDirImpl`
mod testing {
    #![allow(dead_code)]
    use super::{ CacheDirImpl, CacheDirOperations };
    fn is_trait_implemented<T: CacheDirOperations>(_cache_dir_impl: T) {}
    fn assert_trait_impl() { is_trait_implemented(CacheDirImpl); }
}
