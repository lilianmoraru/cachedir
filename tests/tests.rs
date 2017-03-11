extern crate cachedir;

use cachedir::CacheDirConfig;

use std::env;
use std::path::{ Path, PathBuf };

#[test]
#[cfg(any(unix, target_os = "redox"))]
fn create_user_cache() {
    let expected_cache_dir = env::home_dir()
                                 .and_then(|path| {
                                     if path.as_os_str().is_empty() {
                                         None
                                     } else {
                                         if cfg!(not(target_os = "macos")) {
                                             Some(path.join(".cache"))
                                         } else {
                                             Some(path.join("Library/Caches"))
                                         }
                                     }
                                 })
                                 .or_else(|| {
                                     if cfg!(target_os = "emscripten") {
                                         Some(PathBuf::from("/var/cache"))
                                     } else {
                                         None
                                     }
                                 });

    if expected_cache_dir.is_none() {
        // This test will fail in this case, so we will ignore it
    } else {
        let expected_cache_dir = expected_cache_dir.unwrap()
                                                   .join("__cachedir_test_create_user_cache");
        let cache_dir: PathBuf = CacheDirConfig::new("__cachedir_test_create_user_cache")
                                                .get_cache_dir().unwrap()
                                                .into();
        assert_eq!(expected_cache_dir, cache_dir);
    }
}

#[test]
#[cfg(windows)]
fn create_user_cache() {
    let mut cache_dirs: Vec<PathBuf> = Vec::with_capacity(3);

    fn add_env_path(cache_dirs:    &mut Vec<PathBuf>,
                    env_var:       &str) {
        env::var_os(env_var)
            .and_then(|path| {
                if !path.is_empty() {
                    cache_dirs.push(PathBuf::from(path))
                }
            });
    }

    add_env_path(&mut cache_dirs, "LOCALAPPDATA");
    add_env_path(&mut cache_dirs, "APPDATA");

    env::home_dir()
        .and_then(|path| {
            if !path.as_os_str().is_empty() {
                cache_dirs.push(path.join("Cache"))
            }
        });

    if cache_dirs.is_empty() {
        // This test will fail in this case, so we will ignore it
    } else {
        let expected_cache_dir = cache_dirs[0].join("__cachedir_test_create_user_cache");
        let cache_dir: PathBuf = CacheDirConfig::new("__cachedir_test_create_user_cache")
                                                .get_cache_dir().unwrap()
                                                .into();
        assert_eq!(expected_cache_dir, cache_dir);
    }
}

#[test]
#[cfg(any(unix, windows))]
fn create_tmp_cache() {
    if cfg!(unix) {
        let expected_cache_dir = PathBuf::from("/var/tmp/__cachedir_test_create_tmp_cache");
        let cache_dir: PathBuf = CacheDirConfig::new("__cachedir_test_create_tmp_cache")
                                                .tmp_cache(true)
                                                .get_cache_dir().unwrap()
                                                .into();
        assert_eq!(expected_cache_dir, cache_dir);
    } else {
        let temp_dir = env::temp_dir();

        if temp_dir.as_os_str().is_empty() {
            // This test will fail in this case, so we will ignore it
        } else {
            let expected_cache_dir = temp_dir.join("__cachedir_test_create_tmp_cache");
            let cache_dir: PathBuf = CacheDirConfig::new("__cachedir_test_create_tmp_cache")
                                                    .tmp_cache(true)
                                                    .get_cache_dir().unwrap()
                                                    .into();
            assert_eq!(expected_cache_dir, cache_dir);
        }
    }
}

#[test]
#[cfg_attr(not(target_os = "linux"), should_panic)]
fn create_mem_cache() {
    let dev_shm = Path::new("/dev/shm");
    let run_shm = Path::new("/run/shm");

    let expected_cache_dir: Option<PathBuf> = if dev_shm.is_dir() {
        Some(PathBuf::from(dev_shm))
    } else if run_shm.is_dir() {
        Some(PathBuf::from(run_shm))
    } else {
        None
    };

    if expected_cache_dir.is_none() {
        // This test will fail in this case, so we will ignore it
    } else {
        let expected_cache_dir = expected_cache_dir.unwrap()
                                                   .join("__cachedir_test_create_mem_cache");
        let cache_dir: PathBuf = CacheDirConfig::new("__cachedir_test_create_mem_cache")
                                                .mem_cache(true)
                                                .get_cache_dir().unwrap()
                                                .into();
        assert_eq!(expected_cache_dir, cache_dir);
    }
}
