//! The directory a test writes into.

/// A scratch directory per test name, so the suite leaves nothing in a home directory.
pub fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("nox-shield-tests").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a writable scratch directory");
    dir
}
