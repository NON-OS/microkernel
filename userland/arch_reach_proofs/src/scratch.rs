// NONOS Operating System (AGPL-3.0-or-later)
//! A throwaway tree for one test, removed when it goes out of scope.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) struct Scratch(PathBuf);

static NEXT: AtomicUsize = AtomicUsize::new(0);

impl Scratch {
    pub(crate) fn new() -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("arch_reach_{}_{n}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("src")).unwrap();
        Self(dir)
    }

    pub(crate) fn root(&self) -> &Path {
        &self.0
    }

    /// Write `body` at `rel` under the root, making its directories.
    pub(crate) fn put(&self, rel: &str, body: &str) {
        let path = self.0.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
