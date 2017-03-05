use std::path::PathBuf;
use std::io::{ self, Error, ErrorKind };

#[inline]
pub fn user_cache_supported() -> bool {
    true
}

#[inline]
pub fn system_cache_supported() -> bool {
    false
}

pub fn user_cache_path() -> io::Result<PathBuf> {
    unimplemented!() // TODO
}

pub fn system_cache_path() -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::NotFound,
                   "Searching for system-wide cache directories on \"Redox\" \
                   is not supported"))
}
