//! The fresh addresses OP_FRESH_ADDRESS handed out (v1.4.51), each with its
//! account index, in a file beside the account book. It holds addresses and
//! small numbers and no key: SIGN_FROM_FRESH signs only for an address found
//! here, and derives that index's key again to check it.

use std::path::{Path, PathBuf};

use crate::error::{StoreError, WalletError};

/// The highest index a handed-out address may have: one a refund or a
/// channel at a time for years, and a bound a damaged file cannot pass.
pub const MAX_HANDED: u32 = 4096;

static WRITING: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn file(dir: &Path) -> PathBuf {
    dir.join("fresh")
}

fn hex(address: &[u8; 20]) -> String {
    address.iter().map(|b| format!("{b:02x}")).collect()
}

/// Every (index, address) kept; a line that does not read is skipped.
fn read(dir: &Path) -> Vec<(u32, String)> {
    let text = std::fs::read_to_string(file(dir)).unwrap_or_default();
    let line = |l: &str| {
        let (i, a) = l.split_once(' ')?;
        let i = i.parse::<u32>().ok().filter(|i| *i <= MAX_HANDED)?;
        (a.len() == 40).then(|| (i, String::from(a)))
    };
    text.lines().filter_map(line).collect()
}

/// The highest index handed out, if any.
pub fn highest_handed(dir: &Path) -> Option<u32> {
    read(dir).into_iter().map(|(i, _)| i).max()
}

/// The index `address` was handed out at, if it was.
pub fn handed_index(dir: &Path, address: &[u8; 20]) -> Option<u32> {
    let want = hex(address);
    read(dir).into_iter().find(|(_, a)| *a == want).map(|(i, _)| i)
}

/// Keep `address` as handed out at `index`; written to a side file and
/// renamed, so a crash leaves the old list or the new one.
pub fn keep_handed(dir: &Path, index: u32, address: &[u8; 20]) -> Result<(), WalletError> {
    // The shell's worker and its service loop both keep addresses: one at a time.
    let _one = WRITING.lock().map_err(|_| WalletError::Store { source: StoreError::Io })?;
    let mut all = read(dir);
    if all.iter().any(|(i, a)| *i == index && *a == hex(address)) {
        return Ok(());
    }
    all.retain(|(i, _)| *i != index);
    all.push((index, hex(address)));
    let text: String = all.iter().map(|(i, a)| format!("{i} {a}\n")).collect();
    let io = |_| WalletError::Store { source: StoreError::Io };
    std::fs::create_dir_all(dir).map_err(io)?;
    let side = file(dir).with_extension("new");
    std::fs::write(&side, text).map_err(io)?;
    std::fs::rename(side, file(dir)).map_err(io)
}
