cachedir
-
[![Build Status](https://travis-ci.org/lilianmoraru/cachedir.svg?branch=master)](https://travis-ci.org/lilianmoraru/cachedir)

[Documentation](https://docs.rs/cachedir)

---

A Rust library that helps with cache directories creation in a system-agnostic way.

**Note:** even though the crate is at version `0.1`, it should be stable
and its API is not expected to change soon.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
cachedir = "0.1"
```

## Example
```rust
extern crate cachedir;

use cachedir::CacheDirConfig;

fn main() {
    let cache_dir = CacheDirConfig::new("CacheName")
                                   .get_cache_dir()
                                   .unwrap();

    println!("{}", cache_dir.display());
}
```

This creates `CacheName` into the user's cache directory.<br/>
For more information on the types of caches and code examples, please check the [documentation](https://docs.rs/cachedir).