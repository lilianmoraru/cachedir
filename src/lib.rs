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

pub struct CacheDirConfig<'a, 'b> {
    cache_name:     &'a path::Path,
    app_cache_path: Option<&'b path::Path>,
    app_cache:      bool,
    // wasted an hour on this, obsessing about "user" not being 3 characters aligned,
    // but "usr" is not very clear(for non-Unix users) and does not sound as well when pronouncing it
    user_cache:     bool,
    sys_cache:      bool,
    tmp_cache:      bool,
    mem_cache:      bool
}

impl<'a, 'b> CacheDirConfig<'a, 'b> {
    pub fn new<S: AsRef<OsStr> + ?Sized>(cache_name: &'a S) -> CacheDirConfig<'a, 'b> {
        CacheDirConfig {
            cache_name:     path::Path::new(cache_name),
            app_cache_path: None,
            app_cache:      false,
            user_cache:     false,
            sys_cache:      false,
            tmp_cache:      false,
            mem_cache:      false
        }
    }

    pub fn app_cache_path<S: AsRef<OsStr> + ?Sized>(&mut self,
                                                    path: &'b S) -> &mut CacheDirConfig<'a, 'b> {
        self.app_cache_path = Some(path::Path::new(path));
        self.app_cache      = true;
        self
    }

    pub fn app_cache(&mut self, value: bool)  -> &mut CacheDirConfig<'a, 'b> {
        self.app_cache = value;
        if self.app_cache_path.is_none() && self.app_cache {
            self.app_cache_path = if cfg!(not(windows)) {
                Some(path::Path::new(".cache"))
            } else {
                Some(path::Path::new("Cache"))
            };
        }
        self
    }

    pub fn user_cache(&mut self, value: bool) -> &mut CacheDirConfig<'a, 'b> {
        self.user_cache = value;
        self
    }

    pub fn sys_cache(&mut self, value: bool)  -> &mut CacheDirConfig<'a, 'b> {
        self.sys_cache = value;
        self
    }

    pub fn tmp_cache(&mut self, value: bool)  -> &mut CacheDirConfig<'a, 'b> {
        self.tmp_cache = value;
        self
    }

    pub fn mem_cache(&mut self, value: bool)  -> &mut CacheDirConfig<'a, 'b> {
        self.mem_cache = value;
        self
    }

    pub fn try_all_caches(&mut self) -> &mut CacheDirConfig<'a, 'b> {
        // We don't use the `app_cache`(with its fallback to ".cache" and "Cache") function
        // for this one because we assume that the user prefers system-wide directories and because
        // he might not expect to use an application cache by default, if he didn't ask for it
        if self.app_cache_path.is_some() { self.app_cache = true };
        self.user_cache = true;
        self.sys_cache  = true;
        self.tmp_cache  = true;
        self.mem_cache  = true;
        self
    }

    pub fn get_cache_dir(&self) -> io::Result<CacheDir> {
        match sys_cache::create_cache_dir(&self) {
            Ok(path_buf) => Ok( CacheDir { path: path_buf } ),
            Err(err)     => Err(err)
        }
    }
}
