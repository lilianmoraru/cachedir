pub use self::cache_impl::*;

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

// Making sure that we have all the functions
mod testing {
    #[allow(unused_imports)]
    use super::{ user_cache_supported, system_cache_supported,
                 user_cache_path,      system_cache_path };
}