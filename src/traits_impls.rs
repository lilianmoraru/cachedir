use super::CacheDir;

use std::ops;
use std::borrow::Borrow;
use std::convert;

use std::path;
use std::ffi;

impl ops::Deref for CacheDir {
    type Target = path::PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl convert::Into<path::PathBuf> for CacheDir {
    fn into(self) -> path::PathBuf {
        self.path
    }
}

impl convert::Into<ffi::OsString> for CacheDir {
    fn into(self) -> ffi::OsString {
        self.path.into()
    }
}

impl Borrow<path::PathBuf> for CacheDir {
    fn borrow(&self) -> &path::PathBuf {
        &self.path
    }
}

impl Borrow<path::Path> for CacheDir {
    fn borrow(&self) -> &path::Path {
        self.path.borrow()
    }
}

impl Borrow<ffi::OsStr> for CacheDir {
    fn borrow(&self) -> &ffi::OsStr {
        self.path.as_ref()
    }
}

impl AsRef<path::PathBuf> for CacheDir {
    fn as_ref(&self) -> &path::PathBuf {
        &self.path
    }
}

impl AsRef<path::Path> for CacheDir {
    fn as_ref(&self) -> &path::Path {
        self.path.as_ref()
    }
}

impl AsRef<ffi::OsStr> for CacheDir {
    fn as_ref(&self) -> &ffi::OsStr {
        self.path.as_ref()
    }
}

impl<'a> IntoIterator for &'a CacheDir {
    type Item = &'a ffi::OsStr;
    type IntoIter = path::Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}
