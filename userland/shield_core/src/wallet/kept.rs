//! Every spend still on its way, not only the last. A new spend's hand-off is written where the
//! last one's was, so the last one is moved first into `earlier/<n>`, numbered in the order they
//! were proved, with its proof, its notes and its publication record. Each is followed, published
//! again and offered to its owner as the last one is, and goes once it lands or is taken back.

use crate::error::{StoreError, WalletError};
use std::path::{Path, PathBuf};

/// The folder under `export` the earlier hand-offs are kept in.
pub const EARLIER: &str = "earlier";
/// The hand-off of the spend proved last, under `export`.
pub const HANDOFF: &str = "handoff";
/// Written in a hand-off once its owner sent the settlement themselves: from then on it is
/// followed to the block and never handed to a lander again.
pub const SETTLING: &str = "settling";
/// The file that makes a folder a hand-off.
const PROOF: &str = "spend.proof";

/// The earlier hand-offs under `export`, oldest first, by their number.
pub fn earlier(export: &Path) -> Vec<(String, PathBuf)> {
    let Ok(read) = std::fs::read_dir(export.join(EARLIER)) else { return Vec::new() };
    let mut found: Vec<(u64, PathBuf)> = read
        .filter_map(Result::ok)
        .filter_map(|e| {
            let n = e.file_name().to_str()?.parse::<u64>().ok()?;
            let dir = e.path();
            dir.join(PROOF).is_file().then_some((n, dir))
        })
        .collect();
    found.sort_by_key(|(n, _)| *n);
    found.into_iter().map(|(n, dir)| (n.to_string(), dir)).collect()
}

/// The earlier hand-off named `id`, if it is one.
pub fn earlier_dir(export: &Path, id: &str) -> Option<PathBuf> {
    earlier(export).into_iter().find(|(n, _)| n == id).map(|(_, dir)| dir)
}

/// Move the last spend's hand-off out of the way of a new one, keeping it under the next number.
/// Nothing to move is not an error. Returns the number it was kept under.
pub fn keep_last(export: &Path) -> Result<Option<String>, WalletError> {
    let last = export.join(HANDOFF);
    if !last.join(PROOF).is_file() {
        return Ok(None);
    }
    let next = earlier(export).last().and_then(|(n, _)| n.parse::<u64>().ok()).map_or(0, |n| n + 1);
    std::fs::create_dir_all(export.join(EARLIER)).map_err(|_| StoreError::Io)?;
    let to = export.join(EARLIER).join(next.to_string());
    std::fs::rename(&last, &to).map_err(|_| StoreError::Io)?;
    Ok(Some(next.to_string()))
}

/// Forget a hand-off whose spend landed, went another way, or was taken back.
pub fn forget(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
}

/// Mark that the owner sent this spend's settlement themselves.
pub fn mark_settling(dir: &Path) -> Result<(), WalletError> {
    std::fs::write(dir.join(SETTLING), b"1").map_err(|_| StoreError::Io.into())
}

pub fn settling(dir: &Path) -> bool {
    dir.join(SETTLING).is_file()
}

#[cfg(test)]
#[path = "kept_test.rs"]
mod kept_test;
