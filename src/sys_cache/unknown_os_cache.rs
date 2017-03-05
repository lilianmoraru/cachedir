use std::path::PathBuf;
use std::io::{ self, Error, ErrorKind };

#[inline]
pub fn user_cache_supported() -> bool {
    false
}

#[inline]
pub fn system_cache_supported() -> bool {
    false
}

pub fn user_cache_path() -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::NotFound,
                   "cachedir does not support this OS"))
}

pub fn system_cache_path() -> io::Result<PathBuf> {
    Err(Error::new(ErrorKind::NotFound,
                   "cachedir does not support this OS"))
}
