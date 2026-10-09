//! One spend, end to end on this device: anchor to the pool's newest root,
//! pick the notes, check the pool's rules, prove and rank check, seal the two
//! outputs, and write the hand-off a relayer settles. The inputs are marked
//! pending before the call returns, so they cannot be spent twice while the
//! relayer is at work.

use super::{build::request, Destination, Order};
use crate::discovery::note_commitments;
use crate::error::{ProveError, WalletError};
use crate::net::pool::ACTIVE;
use crate::notes::{commitment, NoteStatus};
use crate::prover::launch::prove::prove_spend;
use crate::prover::{pool_hasher, Cancel};
use crate::store::Row;
use crate::wallet::anchor::anchor_where;
use crate::wallet::handoff::{seal_outputs, write_handoff};
use crate::wallet::scan_history::view;
use crate::wallet::sync_chain::History;
use crate::wallet::Session;
use std::path::{Path, PathBuf};

/// What a spend left behind: the file for the relayer, and what it cost.
pub struct SpendOutcome {
    pub handoff: Vec<PathBuf>,
    pub weakened: Vec<&'static str>,
}

pub fn spend(
    session: &mut Session,
    history: &History,
    order: &Order,
    dir: &Path,
    cancel: &Cancel,
) -> Result<SpendOutcome, WalletError> {
    let committed = note_commitments(&view(&history.committed));
    let allowed = crate::wallet::registry::allows(&history.registered);
    let anchor = anchor_where(&committed, &view(&history.roots), &allowed)
        .map_err(|_| ProveError::RootNotPublished)?;
    let need = order.amount.checked_add(order.fee).ok_or(WalletError::Amount)?;
    let held = session.state().held().into_iter();
    let held: Vec<_> = held.filter(|n| crate::store::pool::on_pool(&committed, n)).collect();
    let ripe = super::ripe::ripe(&history.committed, history.head);
    let picked = super::pick::pick_ripe(&held, &anchor, order, need, &ripe)?;
    let own = session.account().address().spend_pk;
    let req = request(&ACTIVE, &anchor, &picked, order, own)?;
    let cache = crate::prover::launch::cache::read(dir);
    let secret = zeroize::Zeroizing::new(session.account().sk().map(|f| f.value()));
    let [a, b] = &picked.notes;
    let proof = prove_spend(&req, &secret, [a, b], cache.as_deref(), cancel)?;
    if let Some(built) = &proof.new_cache {
        crate::prover::launch::cache::keep(dir, built);
    }
    let own_ek = session.account().receive().encapsulation_key();
    let payee_ek = match &order.to {
        Destination::Wallet { sealed_to, .. } => **sealed_to,
        Destination::Withdraw { .. } => own_ek,
    };
    let sealed = seal_outputs(&proof, &payee_ek, &own_ek)?;
    // The last spend may still be on its way: it is kept, and followed, under `earlier`.
    let export = dir.join("export");
    crate::wallet::kept::keep_last(&export)?;
    let handoff = write_handoff(&export.join(crate::wallet::kept::HANDOFF), &proof, &sealed)?;
    for note in picked.notes.iter().filter(|n| n.value > 0) {
        let cm = commitment(&pool_hasher(), &note.note()).map(|f| f.value());
        session.record(Row::Status { cm, status: NoteStatus::Pending })?;
    }
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
    Ok(SpendOutcome { handoff, weakened: proof.weakened })
}
