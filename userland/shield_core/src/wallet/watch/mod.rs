//! View-only accounts, each from a view key somebody shared. They scan like any account, keep
//! their own sealed note log, and have no send: a view key holds no spend secret.

mod edit;
mod file;
mod list;

use crate::keys::view_key_read::ReadViewKey;
use crate::keys::Viewer;
use crate::store::{NoteLog, StoreState};
use std::path::PathBuf;
use zeroize::Zeroizing;

/// The context of the key that seals the view key file and names each watched log.
pub(crate) const WATCH_CONTEXT: &str = "nox-shield 2026 watching key v1";

pub(crate) struct Watched {
    pub(crate) key: ReadViewKey,
    pub(crate) text: Zeroizing<String>,
    pub(crate) log: NoteLog,
    pub(crate) state: StoreState,
}

impl Watched {
    pub(crate) fn viewer(&self) -> Viewer<'_> {
        let nk = self.key.nk.map(|w| w.map(nonos_stark::field::Fp::from_u64));
        Viewer { spend_pk: self.key.spend_pk, receive: &self.key.receive, nk }
    }
}

/// Every view-only account of an unlocked wallet, with the key their files are sealed under.
pub(crate) struct Watching {
    root: Zeroizing<[u8; 32]>,
    file: PathBuf,
    notes: PathBuf,
    pub(crate) list: Vec<Watched>,
}
