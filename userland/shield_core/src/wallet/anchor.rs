//! The root a spend is proved against: the newest `RootCommitted`, rebuilt from its leaves.

use crate::discovery::Log;
use crate::prover::launch::tree::pool_root;
use std::collections::BTreeMap;

/// A committed root and the leaves it covers, as field limbs.
pub struct Anchor {
    pub root: [u64; 4],
    pub leaves: Vec<[u64; 4]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorRefusal {
    /// No root has been committed yet: `commitRoot` first.
    NoRoot,
    /// The history is missing a leaf the root covers.
    HistoryShort,
    /// The leaves do not rebuild to the root the pool published.
    TreeMismatch,
}

/// The newest committed root `allowed` accepts, such as one the association registry holds.
pub fn anchor_where(
    committed: &BTreeMap<u64, [u8; 32]>,
    roots: &[Log],
    allowed: &dyn Fn(&[u8; 32]) -> bool,
) -> Result<Anchor, AnchorRefusal> {
    let (root, count) = roots
        .iter()
        .filter_map(root_of)
        .filter(|(root, _)| allowed(root))
        .max_by_key(|(_, n)| *n)
        .ok_or(AnchorRefusal::NoRoot)?;
    let leaves = (0..count)
        .map(|i| committed.get(&i).map(limbs))
        .collect::<Option<Vec<_>>>()
        .ok_or(AnchorRefusal::HistoryShort)?;
    let root = limbs(&root);
    (pool_root(&leaves) == root)
        .then_some(Anchor { root, leaves })
        .ok_or(AnchorRefusal::TreeMismatch)
}

impl Anchor {
    /// Whether the note at `leaf` is under this root, and so spendable now.
    pub fn covers(&self, leaf: u64) -> bool {
        usize::try_from(leaf).is_ok_and(|l| l < self.leaves.len())
    }
}

/// `RootCommitted`: the root in topic 1, the leaf count in the data word.
fn root_of(log: &Log) -> Option<([u8; 32], u64)> {
    let [_event, root] = log.topics else { return None };
    let mut count = [0u8; 8];
    count.copy_from_slice(log.data.get(24..32)?);
    Some((*root, u64::from_be_bytes(count)))
}

pub(crate) fn limbs(wire: &[u8; 32]) -> [u64; 4] {
    let mut out = [0u64; 4];
    for (limb, chunk) in out.iter_mut().zip(wire.rchunks_exact(8)) {
        let mut b = [0u8; 8];
        b.copy_from_slice(chunk);
        *limb = u64::from_be_bytes(b);
    }
    out
}

#[cfg(test)]
#[path = "anchor_test.rs"]
mod anchor_test;
