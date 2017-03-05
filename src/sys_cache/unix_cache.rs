use std::path::PathBuf;
use std::io::{ self, Error, ErrorKind };

#[inline]
pub fn user_cache_supported() -> bool {
    true // On Android, iOS, NaCl and Emscripten lets hope that $HOME is defined
}

#[inline]
#[cfg(any(target_os = "android",
          target_os = "ios",
          target_os = "nacl",
          target_os = "emscripten"))]
pub fn system_cache_supported() -> bool {
    false
}

#[inline]
#[cfg(not(any(target_os = "android",
              target_os = "ios",
              target_os = "nacl",
              target_os = "emscripten")))]
pub fn system_cache_supported() -> bool {
    true
}

pub fn user_cache_path() -> io::Result<PathBuf> {
    unimplemented!() // TODO
}

pub fn system_cache_path() -> io::Result<PathBuf> {
    unimplemented!() // TODO
}
