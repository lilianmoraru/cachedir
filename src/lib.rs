//! This crate helps with cache directories creation in a system-agnostic way.
//!
//! The [`CacheDirConfig`] type helps define the desired location(s) where attempts to create
//! the cache directory should be made and also helps with the creation itself.
//!
//! The [`CacheDir`] type holds the path to the created cache directory, obtained with the help of
//! [`CacheDirConfig`], if it succeeded to create the directory.
//!
//! [`CacheDir`] derefs to [`PathBuf`] and implements most of the same traits that [`PathBuf`] does.
//!
//! Code examples:
//!
//! - [`Creating an application cache with app_cache_path(custom path)`]
//!
//! - [`Creating an application cache without app_cache_path`]
//!
//! - [`Creating a user cache(the default)`]
//!
//! - [`Creating a system-wide cache`]
//!
//! - [`Creating a tmp cache`]
//!
//! - [`Creating a memory cache`]
//!
//! - [`Using all default fallbacks(without creating an application cache)`]
//!
//! - [`Using all default fallbacks(with automatic creation of the application cache)`]
//!
//! [`Jump to the list of structs`]
//!
//! [`CacheDir`]: struct.CacheDir.html
//! [`CacheDirConfig`]: struct.CacheDirConfig.html
//! [`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
//! [`Creating an application cache with app_cache_path(custom path)`]: index.html#creating-an-application-cache-with-app_cache_pathcustom-path
//! [`Creating an application cache without app_cache_path`]: index.html#creating-an-application-cache-without-app_cache_path
//! [`Creating a user cache(the default)`]: index.html#creating-a-user-cachethe-default
//! [`Creating a system-wide cache`]: index.html#creating-a-system-wide-cache
//! [`Creating a tmp cache`]: index.html#creating-a-tmp-cache
//! [`Creating a memory cache`]: index.html#creating-a-memory-cache
//! [`Using all default fallbacks(without creating an application cache)`]: index.html#using-all-default-fallbackswithout-creating-an-application-cache
//! [`Using all default fallbacks(with automatic creation of the application cache)`]: index.html#using-all-default-fallbackswith-automatic-creation-of-the-application-cache
//! [`Jump to the list of structs`]: index.html#structs
//! [`To the top ⤴`]: index.html
//!
//! # Examples
//!
//! ### Creating an `application cache` **with** `app_cache_path`(custom path)
//! [`To the top ⤴`]
//!
//! ```
//! use cachedir::CacheDirConfig;
//! use std::path::PathBuf;
//! use std::env::current_dir;
//!
//! let current_dir = current_dir().unwrap();
//! let app_cache = CacheDirConfig::new("example/path")
//!                                .app_cache_path(current_dir.as_path())
//!                                .get_cache_dir().unwrap();
//!
//! assert_eq!(current_dir.join("example/path"),
//!            app_cache.into_path_buf());
//! ```
//!
//! ### Creating an `application cache` **without** `app_cache_path`
//! [`To the top ⤴`]
//!
//! ```
//! use cachedir::CacheDirConfig;
//! use std::path::PathBuf;
//! use std::env::current_dir;
//!
//! let app_cache = CacheDirConfig::new("example")
//!                                .app_cache(true)
//!                                .get_cache_dir().unwrap();
//!
//! let current_dir = current_dir().unwrap();
//! if cfg!(not(windows)) {
//!     assert_eq!(current_dir.join(".cache").join("example"),
//!                app_cache.into_path_buf());
//! } else {
//!     assert_eq!(current_dir.join("Cache").join("example"),
//!                app_cache.into_path_buf());
//! }
//! ```
//!
//! ### Creating a `user cache`(the default)
//! [`To the top ⤴`]
//!
//! ```
//! use cachedir::CacheDirConfig;
//! use std::path::PathBuf;
//! use std::env;
//!
//! // User cache is the default
//! let user_cache: PathBuf = CacheDirConfig::new("example")
//!                                          .get_cache_dir().unwrap()
//!                                          .into(); // `CacheDir` implements `Into<PathBuf>`
//!
//! let assert_user_cache = CacheDirConfig::new("example")
//!                                        .user_cache(true)
//!                                        .get_cache_dir().unwrap()
//!                                        .into_path_buf(); // -> PathBuf
//!
//! assert_eq!(assert_user_cache, user_cache);
//!
//! let expected_cache_dir: PathBuf;
//! #[cfg(any(unix, target_os = "redox"))]
//! {
//!     let home_dir = env::home_dir();
//!
//!     #[cfg(not(any(target_os = "emscripten", target_os = "macos")))]
//!     {
//!         expected_cache_dir = home_dir.unwrap().join(".cache").join("example");
//!     }
//!
//!     #[cfg(target_os = "emscripten")]
//!     {
//!         expected_cache_dir = if home_dir.is_none() {
//!             PathBuf::from("/var/cache").join("example")
//!         } else {
//!             home_dir.unwrap().join(".cache").join("example")
//!         };
//!     }
//!
//!     #[cfg(target_os = "macos")]
//!     {
//!         expected_cache_dir = home_dir.unwrap().join("Library/Caches").join("example");
//!     }
//! }
//!
//! #[cfg(windows)]
//! {
//!     let local_app_data = PathBuf::from(env::var_os("LOCALAPPDATA").unwrap());
//!     expected_cache_dir = local_app_data.join("example");
//! }
//!
//! assert_eq!(expected_cache_dir, user_cache);
//! ```
//!
//! ### Creating a `system-wide cache`
//! [`To the top ⤴`]
//!
//! ```no_run
//! use cachedir::CacheDirConfig;
//! use std::path::PathBuf;
//!
//! let cache_dir = CacheDirConfig::new("example")
//!                                .sys_cache(true)
//!                                .get_cache_dir().unwrap();
//!
//! let expected_cache_dir: PathBuf;
//! #[cfg(unix)]
//! {
//!     #[cfg(not(target_os = "macos"))]
//!     { expected_cache_dir = PathBuf::from("/var/cache").join("example"); }
//!
//!     #[cfg(target_os = "macos")]
//!     { expected_cache_dir = PathBuf::from("/Library/Caches").join("example"); }
//! }
//!
//! #[cfg(windows)]
//! {
//!     use std::env;
//!     let program_data = PathBuf::from(env::var_os("ProgramData").unwrap());
//!     expected_cache_dir = program_data.join("example");
//! }
//!
//! assert_eq!(expected_cache_dir, cache_dir.into_path_buf());
//! ```
//!
//! ### Creating a `tmp cache`
//! [`To the top ⤴`]
//!
//! ```
//! use cachedir::CacheDirConfig;
//! use std::path::PathBuf;
//!
//! let cache_dir = CacheDirConfig::new("example")
//!                                .tmp_cache(true)
//!                                .get_cache_dir().unwrap();
//!
//! let expected_cache_dir: PathBuf;
//! #[cfg(unix)]
//! {
//!     // On Unix, we try `/var/tmp` first, because it is more persistent than `/tmp`
//!     expected_cache_dir = PathBuf::from("/var/tmp").join("example");
//! }
//!
//! #[cfg(windows)]
//! {
//!     use std::env::temp_dir;
//!     expected_cache_dir = temp_dir().join("example");
//! }
//!
//! assert_eq!(expected_cache_dir, cache_dir.into_path_buf());
//! ```
//!
//! ### Creating a `memory cache`
//! [`To the top ⤴`]
//!
//! ```
//! use cachedir::CacheDirConfig;
//! use std::path::PathBuf;
//!
//! let cache_dir = CacheDirConfig::new("example/path")
//!                                .mem_cache(true)
//!                                .get_cache_dir().unwrap();
//!
//! // In-memory caches are supported only on Linux
//! if cfg!(target_os = "linux") {
//!     // We try `/dev/shm` before `/run/shm`
//!     assert_eq!(PathBuf::from("/dev/shm").join("example/path"),
//!                cache_dir.into_path_buf());
//! }
//! ```
//!
//! ### Using all default fallbacks(without creating an application cache)
//! [`To the top ⤴`]
//!
//! ```
//! use cachedir::CacheDirConfig;
//! use std::path::PathBuf;
//!
//! let short_version = CacheDirConfig::new("example/path")
//!                                    .try_all_caches()
//!                                    .get_cache_dir().unwrap();
//!
//! let verbose_version = CacheDirConfig::new("example/path")
//!                                      .user_cache(true) // Order here
//!                                      .sys_cache(true)  // does
//!                                      .tmp_cache(true)  // not
//!                                      .mem_cache(true)  // matter
//!                                      .get_cache_dir().unwrap(); // This still is the last call
//!
//! assert_eq!(short_version, verbose_version); // `CacheDir` implements `Eq` and `PartialEq`
//! ```
//! This will try to create the cache directory using these options(listed below).<br/>
//! It falls back to the next option if it fails:<br/>
//!
//! 1. User cache(**persistent** and **doesn't require** elevated rights)
//!
//! 2. System-wide cache(**persistent** and **requires** elevated rights)
//!
//! 3. Tmp cache(**somewhat persistent in /var/tmp** on Unix, **doesn't require** elevated rights)
//!
//! 4. Memory cache(**not persistent** between system restarts, **doesn't require** elevated rights)
//!
//! ### Using all default fallbacks(with automatic creation of the application cache)
//! [`To the top ⤴`]
//!
//! ```no_run
//! use cachedir::CacheDirConfig;
//! use std::path::{ Path, PathBuf };
//! use std::env::current_dir;
//!
//! let one_way = CacheDirConfig::new("example/path")
//!                              .app_cache(true)
//!                              .try_all_caches()
//!                              .get_cache_dir().unwrap();
//!
//! let autocreated_dir = if cfg!(not(windows)) {
//!     Path::new(".cache")
//! } else {
//!     Path::new("Cache")
//! };
//!
//! let app_cache_path = current_dir().unwrap().join(autocreated_dir);
//!
//! // It's OK to use `app_cache(true)` here too
//! let another_way = CacheDirConfig::new("example/path")
//!                                  .app_cache_path(app_cache_path.as_path())
//!                                  .try_all_caches()
//!                                  .get_cache_dir().unwrap();
//!
//! assert_eq!(one_way, another_way);
//!
//! // `app_cache_path` overwrites the default that was set in `app_cache(true)`
//! let yet_another_way = CacheDirConfig::new("example/path")
//!                                      .app_cache(true)
//!                                      .app_cache_path("/tmp/other/path")
//!                                      .try_all_caches()
//!                                      .get_cache_dir().unwrap();
//!
//! assert!(another_way != yet_another_way);
//!
//! // `app_cache(true)` does not overwrite the path of `app_cache_path` if it was set
//! let or_this_way = CacheDirConfig::new("example/path")
//!                                  .app_cache_path("/tmp/other/path")
//!                                  .app_cache(true)
//!                                  .try_all_caches()
//!                                  .get_cache_dir().unwrap();
//!
//! assert_eq!(yet_another_way, or_this_way);
//! ```
//! This will try to create the cache directory using these options(listed below).<br/>
//! It falls back to the next option if it fails:<br/>
//!
//! 1. Application cache
//!
//! 2. User cache(**persistent** and **doesn't require** elevated rights)
//!
//! 3. System-wide cache(**persistent** and **requires** elevated rights)
//!
//! 4. Tmp cache(**somewhat persistent in /var/tmp** on Unix, **doesn't require** elevated rights)
//!
//! 5. Memory cache(**not persistent** between system restarts, **doesn't require** elevated rights)
//!
//! [`To the top ⤴`]

use std::path;
use std::io;
use std::ffi::OsStr;

// Contains the os-agnostic `create_cache_dir` function
mod sys_cache;
mod traits_impls;

pub use traits_impls::*;

/// This structure holds the [`PathBuf`] returned from [`CacheDirConfig`].
///
/// It derefs to [`PathBuf`] and implements most of the same traits as [`PathBuf`].
///
/// # Examples
/// ```
/// # use cachedir::{ CacheDir, CacheDirConfig };
/// # use std::path::{ Path, PathBuf };
/// # use std::ffi::OsStr;
/// let cache_dir: CacheDir = CacheDirConfig::new("example").get_cache_dir().unwrap();
///
/// fn as_path(path: &Path) {}
///
/// fn into_path_buf<P: Into<PathBuf>>(path: P) {
///     let _path_buf: PathBuf = path.into();
/// }
///
/// fn as_ref<P: AsRef<OsStr>>(path: P) {
///     let _os_str: &OsStr = path.as_ref();
/// }
///
/// as_path(cache_dir.as_path());
/// into_path_buf(cache_dir.clone());
/// as_ref(cache_dir.clone());
///
/// println!("{}", cache_dir.display());
/// ```
///
/// [`CacheDirConfig`]: struct.CacheDirConfig.html
/// [`PathBuf`]: https://doc.rust-lang.org/std/path/struct.PathBuf.html
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheDir {
    path: path::PathBuf
}

impl CacheDir {
    /// ```
    /// # use cachedir::CacheDirConfig;
    /// # use std::path::PathBuf;
    /// let path: PathBuf = CacheDirConfig::new("example")
    ///                                    .get_cache_dir().unwrap()
    ///                                    .into_path_buf();
    /// ```
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
