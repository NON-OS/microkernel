//! A spend's end, written into the history once.

use super::super::Wallet;
use crate::wallet::kept::settling;
use crate::wallet::publish::{handoff_nullifiers, Seen};
use std::path::Path;

impl Wallet {
    /// A spend seen at its end, into the history once: landed, or, when another proof spent a
    /// note of it, never to land. A settlement its owner sent lands it even when it shows as
    /// spent another way.
    pub(super) fn note_end(&self, dir: &Path, seen: Seen) {
        use crate::store::activity::{Activity, LANDED, TAKEN_BACK};
        let (kind, tx) = match seen {
            Seen::Landed(tx) => (LANDED, tx.unwrap_or([0; 32])),
            Seen::Elsewhere if settling(dir) => (LANDED, [0; 32]),
            Seen::Elsewhere => (TAKEN_BACK, [0; 32]),
            Seen::Neither => return,
        };
        let Ok([tag, _]) = handoff_nullifiers(dir) else { return };
        let mut a = Activity::new(kind, crate::wallet::history::clock());
        a.tag = tag;
        a.tx = tx;
        let _ = self.with_mut(|s| Ok(crate::wallet::history::once(s, a)?));
    }
}
