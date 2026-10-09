//! Which notes a spend uses, all under the anchor root. The smallest single covering note wins,
//! else the two largest. A single note pairs with a zero dummy.

use crate::notes::{fresh_blinding, NotePlaintext, NoteRecord, NoteStatus};
use crate::wallet::anchor::Anchor;

/// The two inputs, as notes and leaf positions.
pub(super) struct Picked {
    pub notes: [NotePlaintext; 2],
    pub leaves: [u64; 2],
    pub total: u64,
}

pub(super) fn pick(
    held: &[&NoteRecord],
    anchor: &Anchor,
    asset: u64,
    need: u64,
    ready: &dyn Fn(u64) -> bool,
) -> Option<Picked> {
    let mut usable: Vec<&NoteRecord> = held
        .iter()
        .copied()
        .filter(|n| n.status == NoteStatus::Unspent && n.plain.asset_id == asset)
        .filter(|n| anchor.covers(n.leaf_index) && ready(n.leaf_index))
        .collect();
    usable.sort_by_key(|n| n.plain.value);
    if let Some(one) = usable.iter().find(|n| n.plain.value >= need) {
        let dummy =
            NotePlaintext { value: 0, blinding: fresh_blinding().ok()?, ..one.plain.clone() };
        // The dummy is kept off the real note's leaf so the two inputs never coincide.
        let other = u64::from(one.leaf_index == 0);
        return Some(Picked {
            total: one.plain.value,
            leaves: [one.leaf_index, other],
            notes: [one.plain.clone(), dummy],
        });
    }
    let (b, a) = (usable.pop()?, usable.pop()?);
    let total = a.plain.value.checked_add(b.plain.value)?;
    (total >= need).then(|| Picked {
        total,
        leaves: [b.leaf_index, a.leaf_index],
        notes: [b.plain.clone(), a.plain.clone()],
    })
}

/// Ripe notes first. Newer ones pay only when the owner went early, and otherwise the refusal says
/// the notes are too new, not that the balance falls short.
pub(super) fn pick_ripe(
    held: &[&NoteRecord],
    anchor: &Anchor,
    order: &super::Order,
    need: u64,
    ripe: &dyn Fn(u64) -> bool,
) -> Result<Picked, crate::error::WalletError> {
    use crate::error::WalletError;
    let asset = order.asset.id;
    if let Some(picked) = pick(held, anchor, asset, need, ripe) {
        return Ok(picked);
    }
    match pick(held, anchor, asset, need, &|_| true) {
        Some(picked) if order.early => Ok(picked),
        Some(_) => Err(WalletError::TooSoon),
        None => Err(WalletError::Insufficient),
    }
}
