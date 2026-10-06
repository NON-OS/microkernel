//! The prover's request for an order, and the pool's rules checked on it
//! before any proving time is spent.

use super::pick::Picked;
use super::{Destination, Order};
use crate::error::WalletError;
use crate::net::pool::Pool;
use crate::prover::launch::request::SpendRequest;
use crate::wallet::anchor::Anchor;
use crate::wallet::intent_rules::{check_intent, Intent, IntentRefusal, PoolRules};

/// The request, with the payee (or nothing, on a withdrawal) first and the
/// change to this wallet's own key second.
pub(super) fn request(
    pool: &Pool,
    anchor: &Anchor,
    picked: &Picked,
    order: &Order,
    own_pk: [u64; 4],
) -> Result<SpendRequest, WalletError> {
    let asset_id = order.asset.id;
    let spent = order.amount.checked_add(order.fee).ok_or(WalletError::Amount)?;
    let change = picked.total.checked_sub(spent).ok_or(WalletError::Insufficient)?;
    // What leaves this wallet is held to the standard sizes here, before the
    // prover sees it. Change to this wallet's own key is exempt in the prover,
    // since the key derives from the seed's own secrets.
    if !order.asset.allows(order.amount) {
        return Err(WalletError::NotStandard);
    }
    let (first, first_pk, public_amount, recipient) = match &order.to {
        Destination::Wallet { spend_pk, .. } => (order.amount, *spend_pk, 0, [0u8; 20]),
        Destination::Withdraw { recipient } => (0, own_pk, order.amount, *recipient),
    };
    let request = SpendRequest {
        pool_leaves: anchor.leaves.clone(),
        assoc_leaves: anchor.leaves.clone(),
        note_root: anchor.root,
        assoc_root: anchor.root,
        input_pool_index: picked.leaves,
        input_assoc_index: picked.leaves,
        output_values: [first, change],
        output_spend_pk: [first_pk, own_pk],
        asset_id,
        public_amount,
        fee: order.fee,
        clearing_price: 0,
        recipient,
        fee_recipient: order.fee_to,
        not_before: order.not_before,
    };
    let [a, b] = &picked.notes;
    let intent = Intent {
        inputs: [(a.value, a.asset_id, picked.leaves[0]), (b.value, b.asset_id, picked.leaves[1])],
        outputs: [(first, asset_id), (change, asset_id)],
        public_amount,
        fee: order.fee,
        asset_id,
        clearing_price: 0,
        recipient,
        fee_recipient: order.fee_to,
    };
    let rules = PoolRules {
        words_per_intent: pool.words_per_intent,
        max_relay_fee: order.asset.max_relay_fee,
    };
    check_intent(&intent, &rules).map_err(|refusal| match refusal {
        IntentRefusal::FeeExceedsCap => WalletError::FeeTooHigh,
        _ => WalletError::Amount,
    })?;
    Ok(request)
}
