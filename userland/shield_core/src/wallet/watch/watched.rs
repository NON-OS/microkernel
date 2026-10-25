//! One view-only account: its view key, its note log and the state replayed from it.

use crate::keys::view_key_read::ReadViewKey;
use crate::keys::Viewer;
use crate::store::{NoteLog, StoreState};
use zeroize::Zeroizing;

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
