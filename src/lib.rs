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
        self.path
    }
}

pub struct CacheDirConfig<'a, 'b: 'a> {
    cache_name:      &'a path::Path,
    user_parent_dir: Option<&'b path::Path>,
    user_cache:      bool,
    system_cache:    bool,
    tmp_cache:       bool,
    memory_cache:    bool,
}

impl<'a, 'b> CacheDirConfig<'a, 'b> {
    pub fn new<S: AsRef<OsStr> + ?Sized>(cache_name: &'a S) -> CacheDirConfig<'a, 'b> {
        CacheDirConfig {
            cache_name:      path::Path::new(cache_name),
            user_parent_dir: None,
            user_cache:      false,
            system_cache:    false,
            tmp_cache:       false,
            memory_cache:    false,
        }
    }

    pub fn user_parent_dir(&mut self,
                           dir_path: Option<&'b path::Path>) -> &mut CacheDirConfig<'a, 'b> {
        self.user_parent_dir = dir_path;
        self
    }

    pub fn user_cache(&mut self, value: bool)   -> &mut CacheDirConfig<'a, 'b> {
        self.user_cache = value;
        self
    }

    pub fn system_cache(&mut self, value: bool) -> &mut CacheDirConfig<'a, 'b> {
        self.system_cache = value;
        self
    }

    pub fn tmp_cache(&mut self, value: bool)    -> &mut CacheDirConfig<'a, 'b> {
        self.tmp_cache = value;
        self
    }

    pub fn memory_cache(&mut self, value: bool) -> &mut CacheDirConfig<'a, 'b> {
        self.memory_cache = value;
        self
    }

    pub fn try_all_caches(&mut self) -> &mut CacheDirConfig<'a, 'b> {
        self.user_cache   = true;
        self.system_cache = true;
        self.tmp_cache    = true;
        self.memory_cache = true;
        self
    }

    pub fn get_cache_dir(&self) -> io::Result<CacheDir> {
        match sys_cache::create_cache_dir(&self) {
            Ok(path_buf) => Ok( CacheDir { path: path_buf } ),
            Err(err)     => Err(err)
        }
    }
}
