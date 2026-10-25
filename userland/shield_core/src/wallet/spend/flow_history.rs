//! A spend into the history, named by its first nullifier and carrying its change.

use super::super::{Destination, Order};
use crate::prover::launch::prove::LaunchProof;
use crate::wallet::Session;

/// Record the spend `proof` made of `order`.
pub(super) fn record_sent(session: &mut Session, order: &Order, proof: &LaunchProof) {
    // Into the history, named by its first nullifier, with its change, which a sync then
    // knows for this wallet's own and not a payment received.
    use crate::store::activity::{Activity, SENT, WITHDRAWN};
    let kind = match order.to {
        Destination::Wallet { .. } => SENT,
        Destination::Withdraw { .. } => WITHDRAWN,
    };
    let [tag, _] = crate::wallet::publish::nullifiers(&proof.publics);
    let [_, change] = &proof.outputs;
    let mut done = Activity::new(kind, crate::wallet::history::clock());
    done.asset_id = order.asset.id;
    done.value = order.amount;
    done.tag = tag;
    done.cm = crate::wallet::history::cm_of(change);
    // The spend is proved and its notes held either way: a failed write loses the entry only.
    let _ = crate::wallet::history::note(session, done);
}
