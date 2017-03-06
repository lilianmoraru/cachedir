use std::io;
use std::path;

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
    fn create_user_cache_dir(dir_name: &path::Path)   -> io::Result<path::PathBuf>;
    fn create_system_cache_dir(dir_name: &path::Path) -> io::Result<path::PathBuf>;
    fn create_tmp_cache_dir(dir_name: &path::Path)    -> io::Result<path::PathBuf>;
}

// Common function shared between all implementations of the CacheDirOperations trait
fn create_dir(dirs: &[path::PathBuf]) -> io::Result<path::PathBuf> {
    unimplemented!()
}

// Making sure that the `CacheDirOperations` trait was implemented on `CacheDirImpl`
mod testing {
    #![allow(dead_code)]
    use super::{ CacheDirImpl, CacheDirOperations };
    fn is_trait_implemented<T: CacheDirOperations>(_cache_dir_impl: T) {}
    fn assert_trait_impl() { is_trait_implemented(CacheDirImpl); }
}

pub fn create_cache_dir(dir_name:             &path::Path,
                        with_system_fallback: bool,
                        with_tmp_fallback:    bool)
    -> io::Result<path::PathBuf>
{
    unimplemented!()
}
