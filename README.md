cachedir(DEPRECATED), please use dirs instead: https://crates.io/crates/dirs
-
[![Build Status](https://travis-ci.org/lilianmoraru/cachedir.svg?branch=master)](https://travis-ci.org/lilianmoraru/cachedir)
[![Build status](https://ci.appveyor.com/api/projects/status/ir02vrt2unxjjqax/branch/master?svg=true)](https://ci.appveyor.com/project/lilianmoraru/cachedir/branch/master)

[Documentation](https://docs.rs/cachedir)

Status: `finished`

---
A Rust library that helps with cache directories creation in a system-agnostic way.

**Note:** even though the crate is at version `0.1`, it should be stable
and its API is not expected to change soon.<br/><br/>

Dual-licensed under MIT or the [UNLICENSE](http://unlicense.org).

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
