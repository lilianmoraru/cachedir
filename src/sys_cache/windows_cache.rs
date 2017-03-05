use std::path::PathBuf;
use std::io::{ self, Error, ErrorKind };

#[inline]
pub fn user_cache_supported() -> bool {
    true
}

#[inline]
pub fn system_cache_supported() -> bool {
    true
}

pub fn user_cache_path() -> io::Result<PathBuf> {
    unimplemented!() // TODO
}

pub fn system_cache_path() -> io::Result<PathBuf> {
    unimplemented!() // TODO
}
