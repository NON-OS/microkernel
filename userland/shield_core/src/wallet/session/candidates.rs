//! The accounts a restore may find. Restoring derives the next accounts from the seed while it
//! is at hand and keeps them unused, since the seed is dropped before the restore returns. A
//! search then keeps those up to the last one used, and a lock forgets the rest.

use super::book::{Book, MAX_ACCOUNTS};
use super::slot::Slot;
use super::Session;
use crate::custody::Seed;
use crate::error::WalletError;
use crate::wallet::paths::Paths;

impl Session {
    pub(super) fn derive_candidates(
        &mut self,
        paths: &Paths,
        seed: &Seed,
    ) -> Result<(), WalletError> {
        /* One listing of the store's folder says which candidate logs exist, so a restore
         * into a new folder asks the disk once rather than once a candidate. A log there is
         * replayed as ever; a folder that cannot be listed is asked log by log. */
        let held = held_logs(paths);
        self.candidates = (1..MAX_ACCOUNTS)
            .map(|i| match &held {
                Some(names) if !names.contains(&log_name(paths, i)) => {
                    Slot::unwritten(paths, seed, i)
                }
                _ => Slot::open(paths, seed, i),
            })
            .collect::<Result<_, _>>()?;
        Ok(())
    }

    pub(crate) fn candidates(&self) -> &[Slot] {
        &self.candidates
    }

    /// Keep the first `found` candidates as accounts 1 to `found`, and forget the others.
    pub(crate) fn keep_candidates(
        &mut self,
        paths: &Paths,
        found: usize,
    ) -> Result<(), WalletError> {
        self.next_public = self.candidates.get(found).map(|c| c.evm().address());
        let kept: Vec<Slot> = self.candidates.drain(..).take(found).collect();
        self.more.extend(kept);
        Book { count: self.count(), active: self.active }.write(&paths.account)
    }
}

/// The file names in the folder the note logs are kept in, or None when it cannot be listed.
fn held_logs(paths: &Paths) -> Option<Vec<std::ffi::OsString>> {
    let dir = paths.notes_of(1).parent()?.to_path_buf();
    let listed = std::fs::read_dir(dir).ok()?;
    Some(listed.filter_map(|e| e.ok().map(|e| e.file_name())).collect())
}

fn log_name(paths: &Paths, index: u32) -> std::ffi::OsString {
    paths.notes_of(index).file_name().map(std::ffi::OsString::from).unwrap_or_default()
}

#[cfg(test)]
#[path = "candidates_test.rs"]
mod candidates_test;
