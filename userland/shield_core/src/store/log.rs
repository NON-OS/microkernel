//! The sealed append only log. A power cut costs at most the last row.
//! The caller applies a row only after it is written, or a spent note could return after restart.

use super::key::derive;
use super::replay::replay;
use super::state::StoreState;
use crate::custody::Seed;
use crate::error::StoreError;
use nonos_seal::SealState;
use std::path::PathBuf;

/// The sealed append only log.
pub struct NoteLog {
    pub(super) path: PathBuf,
    pub(super) seal: SealState,
}

impl NoteLog {
    /// Open the log at `path` and replay it. A row that does not authenticate stops the replay.
    pub fn open(
        path: impl Into<PathBuf>,
        seed: &Seed,
    ) -> Result<(NoteLog, StoreState), StoreError> {
        NoteLog::open_account(path, seed, 0)
    }

    /// Open the log of account `index`, sealed under that account's store key.
    pub fn open_account(
        path: impl Into<PathBuf>,
        seed: &Seed,
        index: u32,
    ) -> Result<(NoteLog, StoreState), StoreError> {
        NoteLog::open_keyed(path, derive(seed, index))
    }

    /// Open a log sealed under `key`, for a view-only account.
    pub fn open_keyed(
        path: impl Into<PathBuf>,
        key: [u8; 32],
    ) -> Result<(NoteLog, StoreState), StoreError> {
        let path = path.into();
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(_) => return Err(StoreError::Io),
        };
        NoteLog::from_bytes(path, key, &bytes)
    }

    /// The log of account `index` at `path`, known not to exist yet: the same log an open of
    /// a missing file gives, without asking the disk.
    pub fn unwritten_account(
        path: impl Into<PathBuf>,
        seed: &Seed,
        index: u32,
    ) -> Result<(NoteLog, StoreState), StoreError> {
        NoteLog::from_bytes(path.into(), derive(seed, index), &[])
    }

    fn from_bytes(
        path: PathBuf,
        key: [u8; 32],
        bytes: &[u8],
    ) -> Result<(NoteLog, StoreState), StoreError> {
        let (state, next) = replay(bytes, key)?;
        Ok((NoteLog { path, seal: SealState::new(key, next) }, state))
    }
}
