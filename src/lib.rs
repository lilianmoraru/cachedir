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
//! ```no_run
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
//! ```
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

// Traits implementations for `CacheDir`
mod traits_impls;

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

/// This structure helps configure the desired behavior when attempting to create
/// a cache directory and also creates the directory based on that behavior.
///
/// `CacheDirConfig` prioritizes the most persistent destinations first and then the locations
/// that require the least user rights, when attempting to create a cache directory.<br/><br/>
///
/// The default behavior is to create a `user cache`(won't attempt other cache options).
///
/// If another cache option is passed, then the `user cache` default won't be used any more.
///
/// If only [`app_cache_path`] and/or [`app_cache(true)`] is passed, then only the application cache
/// will be created(there won't be attempts to fallback to other cache options).
///
/// Multiple fallbacks can be used by using the [`app_cache_path`], [`app_cache`], [`user_cache`],
/// [`sys_cache`], [`tmp_cache`], [`mem_cache`] and [`try_all_caches`] functions to configure
/// the behavior.
///
/// The order of attempts looks like this(note that unset cache options are skipped):
///
/// 1. Application cache
///
/// 2. User cache(**persistent** and **doesn't require** elevated rights)
///
/// 3. System-wide cache(**persistent** and **requires** elevated rights)
///
/// 4. Tmp cache(**somewhat persistent in /var/tmp** on Unix, **doesn't require** elevated rights)
///
/// 5. Memory cache(**not persistent** between system restarts, **doesn't require** elevated rights)
///
/// Example: If a user sets [`user_cache(true)`], [`sys_cache(true)`] and [`mem_cache(true)`]
/// than `CaheDirConfig` will attempt to create a cache directory in the `user_cache`, than, if it
/// fails(ex: cache directory not found, missing rights, a file with the same name exists, etc...),
/// it will fall-back to the `sys_cache`, in which case, if this also fails, it will attempt as a
/// last resort to create a cache directory in the `mem_cache`.
///
/// [`app_cache_path`]: struct.CacheDirConfig.html#method.app_cache_path
/// [`app_cache(true)`]: struct.CacheDirConfig.html#method.app_cache
/// [`app_cache`]: struct.CacheDirConfig.html#method.app_cache
/// [`user_cache`]: struct.CacheDirConfig.html#method.user_cache
/// [`user_cache(true)`]: struct.CacheDirConfig.html#method.user_cache
/// [`sys_cache`]: struct.CacheDirConfig.html#method.sys_cache
/// [`sys_cache(true)`]: struct.CacheDirConfig.html#method.sys_cache
/// [`tmp_cache`]: struct.CacheDirConfig.html#method.tmp_cache
/// [`mem_cache`]: struct.CacheDirConfig.html#method.mem_cache
/// [`mem_cache(true)`]: struct.CacheDirConfig.html#method.mem_cache
/// [`try_all_caches`]: struct.CacheDirConfig.html#method.try_all_caches
///
/// # Examples
/// ```
/// # use cachedir::CacheDirConfig;
/// let cache_dir = CacheDirConfig::new("example").get_cache_dir().unwrap();
/// ```
/// This will attempt to create the cache directory `example` in the `User Cache`.<br/>
/// Read [`user_cache`] documentation if you want to find more about the paths used for `User Cache`.
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
    /// `cache_name` accepts a path - used to create the cache directory.
    ///
    /// If it *does not exist* at the desired location, `CacheDirConfig` will create it when
    /// calling `get_cache_dir()`, before returning the path to the final cache destination.
    ///
    /// If it *already exists* and it is a directory, `get_cache_dir()` will
    /// return the path to the final cache destination(**note:** in this situation, `CacheDirConfig`
    /// cannot guarantee that you have access to write in the final destination, it just confirms
    /// that the cache directory already exists).
    ///
    /// # Examples
    /// ```
    /// use cachedir::CacheDirConfig;
    /// let cache_config = CacheDirConfig::new("some/path");
    /// ```
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

    /// This function allows to choose a custom path where the cache directory should be created.
    ///
    /// If it *does not exist*, `CacheDirConfig` will attempt to create it.
    ///
    /// If none of the other cache options are selected, this will be the only directory
    /// where `CacheDirConfig` will attempt to create the cache directory.
    ///
    /// Using this function automatically switches `app_cache` to `true`. It can manually be
    /// disabled later by calling `app_cache(false)` on this `CacheDirConfig` object.
    ///
    /// # Examples
    /// ```no_run
    /// use cachedir::CacheDirConfig;
    /// let cache_dir = CacheDirConfig::new("some/path")
    ///                                .app_cache_path("/application/cache")
    ///                                .get_cache_dir();
    /// ```
    pub fn app_cache_path<S: AsRef<OsStr> + ?Sized>(&mut self,
                                                    path: &'b S) -> &mut CacheDirConfig<'a, 'b> {
        self.app_cache_path = Some(path::Path::new(path));
        self.app_cache      = true;
        self
    }

    /// This function tells `CacheDirConfig` if it should attempt to create
    /// an application cache directory.
    ///
    /// ### Non-Windows
    /// If `app_cache_path` is not passed, `CacheDirConfig` will attempt to create the
    /// application cache based on the *current directory + ".cache"*
    ///
    /// If the directory *does not exist*, `CacheDirConfig` will attempt to create it.
    ///
    /// ### Windows
    /// If `app_cache_path` is not passed, `CacheDirConfig` will attempt to create the
    /// application cache based on the *current directory + "Cache"*
    ///
    /// If the directory *does not exist*, `CacheDirConfig` will attempt to create it.
    ///
    /// # Examples
    /// ```
    /// use cachedir::CacheDirConfig;
    /// let cache_dir = CacheDirConfig::new("some/path")
    ///                                .app_cache(true)
    ///                                .get_cache_dir();
    /// ```
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

    /// This function tells `CacheDirConfig` if it should attempt to create
    /// a user cache directory.
    ///
    /// Note that if none of the cache options are passed, than the default
    /// is to use the `User Cache`.
    ///
    /// ### Unix
    /// `CacheDirConfig` will attempt to obtain the `HOME` directory of the user.
    /// If it fails, it will try the next fallback. If a fallback was not configured, it will
    /// return an `std::io::Error` when calling `get_cache_dir`.<br/>
    /// An exception is made for `Emscripten` - in this case it will attempt to create `/var/cache`
    /// as a parent directory for the final cache destination.
    ///
    /// If `HOME` was found, than `CacheDirConfig` will attempt to return or
    /// create(if it is missing) the `.cache` directory from inside the `HOME` directory
    /// on `non-macOS` systems and `Library/Caches` on `macOS`.
    ///
    /// ### Windows
    /// `CacheDirConfig` will attempt to create the cache directory inside these paths(and
    /// fallback to the next if it fails):
    ///
    /// 1. Windows environment variable: `%LOCALAPPDATA%`
    ///
    /// 2. `%APPDATA%`
    ///
    /// 3. Rust's `home_dir` which returns `%HOME%` --ifndef--> `%USERPROFILE%`
    /// --ifndef--> `OS syscall`.<br/>
    /// Since the user's home directory is not a dedicated cache directory, `CacheDirConfig`
    /// will attempt to create the `Cache` directory inside it.
    ///
    /// If it fails, it will try the next fallback. If a fallback was not configured, it will
    /// return an `std::io::Error` when calling `get_cache_dir`.
    ///
    /// ### Redox
    /// `CacheDirConfig` will attempt to obtain the `HOME` directory of the user.
    /// If it fails, it will try the next fallback. If a fallback was not configured, it will
    /// return an `std::io::Error` when calling `get_cache_dir`.
    ///
    /// If `HOME` was found, than `CacheDirConfig` will attempt to return or
    /// create(if it is missing) the `.cache` directory from inside the `HOME` directory.
    ///
    /// # Examples
    /// ```
    /// use cachedir::CacheDirConfig;
    /// let cache_dir = CacheDirConfig::new("some/path")
    ///                                .user_cache(true)
    ///                                .get_cache_dir();
    /// ```
    pub fn user_cache(&mut self, value: bool) -> &mut CacheDirConfig<'a, 'b> {
        self.user_cache = value;
        self
    }

    /// This function tells `CacheDirConfig` if it should attempt to create
    /// a system-wide cache directory.<br/>
    /// **Note:** This might require elevated rights(ex: `superuser`, `Admin`...) in the system.
    ///
    /// ### Unix(non-macOS)
    /// `CacheDirConfig` will attempt to create the cache directory inside `/var/cache`
    /// when calling `get_cache_dir`.
    ///
    /// If it fails, it will try the next fallback. If a fallback was not configured, it will
    /// return an `std::io::Error` when calling `get_cache_dir`.
    ///
    /// ### Unix(macOS)
    /// `CacheDirConfig` will attempt to create the cache directory inside `/Library/Caches`
    /// when calling `get_cache_dir`.
    ///
    /// If it fails, it will try the next fallback. If a fallback was not configured, it will
    /// return an `std::io::Error` when calling `get_cache_dir`.
    ///
    /// ### Windows
    /// `CacheDirConfig` will attempt to create the cache directory inside `%ProgramData%`
    /// when calling `get_cache_dir`.
    ///
    /// If it fails, it will try the next fallback. If a fallback was not configured, it will
    /// return an `std::io::Error` when calling `get_cache_dir`.
    ///
    /// ### Redox
    /// Currently not supported
    ///
    /// # Examples
    /// ```no_run
    /// use cachedir::CacheDirConfig;
    /// let cache_dir = CacheDirConfig::new("some/path")
    ///                                .sys_cache(true)
    ///                                .get_cache_dir();
    /// ```
    pub fn sys_cache(&mut self, value: bool)  -> &mut CacheDirConfig<'a, 'b> {
        self.sys_cache = value;
        self
    }

    /// This function tells `CacheDirConfig` if it should attempt to create
    /// a cache directory inside one of the system's temporary directories.
    ///
    /// ### Unix
    /// `CacheDirConfig` will attempt to create the cache directory in
    /// 2 locations(will use the second location only if the first one fails):
    ///
    /// 1. `/var/tmp`
    ///
    /// 2. Rust's `temp_dir` which is usually(not on Android) `/tmp`
    ///
    /// `/var/tmp` is preferred because it is more persistent between system restarts.<br/>
    /// It is usually automatically cleaned every 30 days by the system.
    ///
    /// If these fail, `CacheDirConfig` will try the next fallback.
    /// If a fallback was not configured, it will return an `std::io::Error` when calling
    /// `get_cache_dir`.
    ///
    /// ### Windows and Redox
    /// `CacheDirConfig` will attempt to create the cache directory inside the `TMP/TEMP` directory.
    ///
    /// The `TMP/TEMP` directory is obtained internally by calling Rust's `temp_dir`.
    ///
    /// If it fails, it will try the next fallback. If a fallback was not configured, it
    /// will return `std::io::Error` when calling `get_cache_dir`.
    ///
    /// # Examples
    /// ```
    /// use cachedir::CacheDirConfig;
    /// let cache_dir = CacheDirConfig::new("some/path")
    ///                                .tmp_cache(true)
    ///                                .get_cache_dir();
    /// ```
    pub fn tmp_cache(&mut self, value: bool)  -> &mut CacheDirConfig<'a, 'b> {
        self.tmp_cache = value;
        self
    }

    /// **Linux and Emscripten only(although, it is OK to call the function on any system)**
    ///
    /// This function tells `CacheDirConfig` if it should attempt to create a cache directory
    /// inside one of these paths:
    ///
    /// 1. `/dev/shm`
    ///
    /// 2. `/run/shm`
    ///
    /// If these fail, when calling `get_cache_dir`, it will return an `std::io::Error`.
    ///
    /// # Examples
    /// ```no_run
    /// use cachedir::CacheDirConfig;
    /// let cache_dir = CacheDirConfig::new("some/path")
    ///                                .mem_cache(true)
    ///                                .get_cache_dir();
    /// ```
    pub fn mem_cache(&mut self, value: bool)  -> &mut CacheDirConfig<'a, 'b> {
        self.mem_cache = value;
        self
    }

    /// This function tells `CacheDirConfig` if it should try to use all the cache fallbacks
    /// when attempting to create the cache directory.
    ///
    /// This does not activate `app_cache` if a path for it was not set.
    ///
    /// # Examples
    /// ```no_run
    /// use cachedir::CacheDirConfig;
    ///
    /// // This will not activate the `app_cache`
    /// let cache_dir1 = CacheDirConfig::new("some/path")
    ///                                .try_all_caches()
    ///                                .get_cache_dir();
    ///
    /// // This will activate `app_cache`
    /// let cache_dir2 = CacheDirConfig::new("some/path")
    ///                                 .app_cache_path("/path/to/app/cache")
    ///                                 .app_cache(false) // try_all_caches will activate it back
    ///                                 .try_all_caches()
    ///                                 .get_cache_dir();
    /// ```
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

    /// This creates the cache directory based on the `CacheDirConfig` configurations.
    ///
    /// The returned `CacheDir` contains the path to the cache directory.
    ///
    /// # Errors
    /// If the directory could not be created, `std::io::Error` is returned.</br>
    /// Calling `.kind()` on it will return the last `std::io::ErrorKind`.<br/>
    /// Calling `.description()` will return the description of all the `CacheDirConfig`
    /// attempts that failed.
    ///
    /// # Examples
    /// ```no_run
    /// use cachedir::CacheDirConfig;
    /// let cache_dir = CacheDirConfig::new("some/path")
    ///                                .get_cache_dir();
    /// ```
    pub fn get_cache_dir(&self) -> io::Result<CacheDir> {
        match sys_cache::create_cache_dir(&self) {
            Ok(path_buf) => Ok( CacheDir { path: path_buf } ),
            Err(err)     => Err(err)
        }
    }
}
