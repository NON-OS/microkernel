//! Folding the rows up into the history: a later row moves an earlier entry on.

use super::{Activity, Entry, Stage};
use super::{
    DEPOSIT_IN, DEPOSIT_SENT, LANDED, RECEIVED, SENT, SETTLED_SELF, TAKEN_BACK, WITHDRAWN,
};

fn some(tx: [u8; 32]) -> Option<[u8; 32]> {
    (tx != [0; 32]).then_some(tx)
}

/// The history the rows fold up to. A later row moves an earlier entry on; one that names no
/// entry, as after a store was restored from the words, starts its own.
pub fn history(rows: &[Activity]) -> Vec<Entry> {
    let mut out: Vec<(Activity, Entry)> = Vec::new();
    for a in rows {
        let spend =
            |e: &(Activity, Entry)| matches!(e.0.kind, SENT | WITHDRAWN) && e.0.tag == a.tag;
        match a.kind {
            DEPOSIT_SENT | SENT | WITHDRAWN => {
                let entry = Entry {
                    kind: a.kind,
                    asset_id: a.asset_id,
                    value: a.value,
                    at: a.at,
                    stage: Stage::OnItsWay,
                    tx: some(a.tx),
                };
                out.push((*a, entry));
            }
            DEPOSIT_IN => {
                if let Some(e) = out.iter_mut().find(|e| e.0.kind == DEPOSIT_SENT && e.0.cm == a.cm)
                {
                    e.1.stage = Stage::Done;
                }
            }
            LANDED => {
                if let Some(e) = out.iter_mut().find(|e| spend(e)) {
                    e.1.stage = Stage::Done;
                    e.1.tx = some(a.tx).or(e.1.tx);
                }
            }
            SETTLED_SELF => {
                if let Some(e) = out.iter_mut().find(|e| spend(e) && e.1.stage == Stage::OnItsWay) {
                    e.1.stage = Stage::Settling;
                    e.1.tx = some(a.tx);
                }
            }
            TAKEN_BACK => {
                if let Some(e) = out.iter_mut().find(|e| spend(e) && e.1.stage != Stage::Done) {
                    e.1.stage = Stage::TakenBack;
                }
            }
            RECEIVED => out.push((
                *a,
                Entry {
                    kind: RECEIVED,
                    asset_id: a.asset_id,
                    value: a.value,
                    at: a.at,
                    stage: Stage::Done,
                    tx: None,
                },
            )),
            _ => {}
        }
    }
    out.into_iter().map(|(_, e)| e).collect()
}
