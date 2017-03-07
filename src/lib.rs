use std::path;
use std::io;
use std::ffi::OsStr;

// Contains the os-agnostic `create_cache_dir` function
mod sys_cache;

pub struct CacheDir {
    path: path::PathBuf
}

impl CacheDir {
    pub fn as_path(&self) -> &path::Path {
        self.path.as_ref()
    }

    pub fn into_path_buf(self) -> path::PathBuf {
        // Note that we take the ownership,
        // so we won't be able to call `as_path` later and panic
        self.path
    }
}

pub struct CacheDirConfig<'a> {
    dir_name:        &'a path::Path,
    system_fallback: bool,
    tmp_fallback:    bool,
}

impl<'a> CacheDirConfig<'a> {
    pub fn new<S: AsRef<OsStr> + ?Sized>(dir_name: &'a S) -> CacheDirConfig<'a> {
        CacheDirConfig {
            dir_name: path::Path::new(dir_name),
            system_fallback: false,
            tmp_fallback: false
        }
    }

//    Will be added later
//    pub fn parent_dir(&mut self, dir: &path::Path) -> &mut CacheDirConfig<'a> {
//
//    }

    pub fn system_fallback(&mut self, value: bool) -> &mut CacheDirConfig<'a> {
        self.system_fallback = value;
        self
    }

    pub fn tmp_fallback(&mut self, value: bool) -> &mut CacheDirConfig<'a> {
        self.tmp_fallback = value;
        self
    }

    pub fn with_all_fallbacks(&mut self) -> &mut CacheDirConfig<'a> {
        self.system_fallback = true;
        self.tmp_fallback = true;
        self
    }

    pub fn get_cache_dir(&self) -> io::Result<CacheDir> {
        use sys_cache::create_cache_dir;

        match create_cache_dir(self.dir_name, self.system_fallback, self.tmp_fallback) {
            Ok(path_buf) => Ok( CacheDir { path: path_buf } ),
            Err(err)     => Err(err)
        }
    }
}
