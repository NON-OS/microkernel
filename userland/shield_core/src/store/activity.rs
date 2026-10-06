//! What this wallet did with the pool, as rows of the store: deposits sent and taken in, spends
//! proved, landed, settled by their owner or taken back, and payments received. Kept beside the
//! notes, sealed the same way, so the history outlives a restart and goes wherever the store
//! goes. A spend is named by its first nullifier, which its follow reads too, and carries its
//! change note's commitment, so the change a sync finds is never taken for a payment received.

/// A deposit sent from the public account: its note's commitment, its transaction.
pub const DEPOSIT_SENT: u8 = 1;
/// A deposit's note seen stored in the pool, by its commitment.
pub const DEPOSIT_IN: u8 = 2;
/// A private payment proved, by its first nullifier, with its change commitment.
pub const SENT: u8 = 3;
/// A withdrawal proved, the same.
pub const WITHDRAWN: u8 = 4;
/// A spend seen on chain, by its first nullifier, with its settlement when named.
pub const LANDED: u8 = 5;
/// A spend's settlement sent by its owner from the public account.
pub const SETTLED_SELF: u8 = 6;
/// A spend's notes taken back: it never lands.
pub const TAKEN_BACK: u8 = 7;
/// A note received from someone else, by its commitment.
pub const RECEIVED: u8 = 8;

/// One thing done, as a row writes it. Unused fields are zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Activity {
    pub kind: u8,
    pub asset_id: u64,
    pub value: u64,
    /// Seconds since 1970 when it was recorded.
    pub at: u64,
    pub tx: [u8; 32],
    /// The spend's first nullifier.
    pub tag: [u8; 32],
    /// A note's commitment: the deposit's, the change's, or the one received.
    pub cm: [u64; 4],
}

impl Activity {
    pub fn new(kind: u8, at: u64) -> Activity {
        Activity { kind, asset_id: 0, value: 0, at, tx: [0; 32], tag: [0; 32], cm: [0; 4] }
    }

    pub fn known(kind: u8) -> bool {
        (DEPOSIT_SENT..=RECEIVED).contains(&kind)
    }
}

/// Where one entry of the history stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// Sent or proved, and not seen on chain yet.
    OnItsWay,
    /// Its owner sent the settlement and it is not seen on chain yet.
    Settling,
    /// In the pool: a deposit stored, a spend landed, a payment received.
    Done,
    /// Taken back: it never landed and its notes are spendable again.
    TakenBack,
}

/// One entry of the history, oldest first.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    /// `DEPOSIT_SENT`, `SENT`, `WITHDRAWN` or `RECEIVED`: what the entry is.
    pub kind: u8,
    pub asset_id: u64,
    pub value: u64,
    pub at: u64,
    pub stage: Stage,
    /// The transaction that put it on chain, when one is known.
    pub tx: Option<[u8; 32]>,
}

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

/// Whether the rows already say the spend `tag` landed or was taken back.
pub fn settled(rows: &[Activity], tag: &[u8; 32]) -> bool {
    rows.iter().any(|a| matches!(a.kind, LANDED | TAKEN_BACK) && a.tag == *tag)
}

/// Whether `cm` is the change of one of this wallet's own spends.
pub fn is_change(rows: &[Activity], cm: &[u64; 4]) -> bool {
    rows.iter().any(|a| matches!(a.kind, SENT | WITHDRAWN) && a.cm == *cm)
}

/// 32 bytes as the four little endian words a row carries, and back.
pub(crate) fn to_quad(b: &[u8; 32]) -> [u64; 4] {
    let mut q = [0u64; 4];
    for (w, chunk) in q.iter_mut().zip(b.chunks_exact(8)) {
        let mut x = [0u8; 8];
        x.copy_from_slice(chunk);
        *w = u64::from_le_bytes(x);
    }
    q
}

pub(crate) fn from_quad(q: &[u64; 4]) -> [u8; 32] {
    let mut b = [0u8; 32];
    for (chunk, w) in b.chunks_exact_mut(8).zip(q) {
        chunk.copy_from_slice(&w.to_le_bytes());
    }
    b
}

#[cfg(test)]
#[path = "activity_test.rs"]
mod activity_test;
