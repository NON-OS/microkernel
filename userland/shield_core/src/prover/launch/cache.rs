//! The periodic cache on disk, one file per circuit, named after the root it must carry. A file
//! another circuit left, the launch one before v2, is removed before a proof, so an upgrade
//! builds its own file and never offers the prover a tree it refuses.

use std::path::{Path, PathBuf};

const PREFIX: &str = "periodic";
const SUFFIX: &str = ".top";

/// The file of this build's circuit: `periodic-`, the first four bytes of its root in hex.
pub fn path(dir: &Path) -> PathBuf {
    let tag: String =
        nox_prover::PERIODIC_ROOT.iter().take(4).map(|b| format!("{b:02x}")).collect();
    dir.join(format!("{PREFIX}-{tag}{SUFFIX}"))
}

/// This circuit's cache when held, after any file of another circuit is removed.
pub fn read(dir: &Path) -> Option<Vec<u8>> {
    let own = path(dir);
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(PREFIX) && name.ends_with(SUFFIX) && entry.path() != own {
            let _ = std::fs::remove_file(entry.path());
        }
    }
    std::fs::read(own).ok()
}

/// Keep the cache a proof built, for the next proof. A failed write costs only time.
pub fn keep(dir: &Path, built: &[u8]) {
    let _ = std::fs::write(path(dir), built);
}
