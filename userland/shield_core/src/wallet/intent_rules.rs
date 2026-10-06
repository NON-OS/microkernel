//! What a spend must satisfy before proving. A transfer publishes no price, which would leak.

use super::intent_limits::{fee_within_cap, in_range, representable};
pub use super::intent_limits::{PoolRules, MAX_FEE_BPS};

/// A spend as the wallet is about to prove it: two inputs in, two outputs out.
#[derive(Clone, Copy, Debug)]
pub struct Intent {
    /// The inputs' values and assets, and their leaf positions, which must differ.
    pub inputs: [(u64, u64, u64); 2],
    /// The outputs' values and assets.
    pub outputs: [(u64, u64); 2],
    pub public_amount: u64,
    pub fee: u64,
    pub asset_id: u64,
    pub clearing_price: u64,
    pub recipient: [u8; 20],
    /// Who is paid the fee: the relayer that submits the proof.
    pub fee_recipient: [u8; 20],
}

/// Why the pool, the prover or the circuit would refuse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntentRefusal {
    MixedAssets,
    SameInputTwice,
    Unbalanced,
    /// A value, public amount or fee above `MAX_VALUE`, or a price outside the field.
    OutOfRange,
    /// An address the pool's statement cannot carry.
    NotRepresentable,
    FeeRecipientWithoutFee,
    FeeWithoutFeeRecipient,
    /// A transfer that names a recipient or a price.
    TransferHasPublicLeg,
    RecipientRequired,
    FeeExceedsCap,
}

/// Check an intent against the pool's, the prover's and the circuit's rules.
pub fn check_intent(it: &Intent, pool: &PoolRules) -> Result<(), IntentRefusal> {
    let assets = it.inputs.iter().map(|i| i.1).chain(it.outputs.iter().map(|o| o.1));
    if assets.into_iter().any(|a| a != it.asset_id) {
        return Err(IntentRefusal::MixedAssets);
    }
    let [(v0, _, l0), (v1, _, l1)] = it.inputs;
    if l0 == l1 {
        return Err(IntentRefusal::SameInputTwice);
    }
    let [(o0, _), (o1, _)] = it.outputs;
    if !in_range(&[v0, v1, o0, o1, it.public_amount, it.fee], it.clearing_price) {
        return Err(IntentRefusal::OutOfRange);
    }
    // Over the integers, as the circuit balances: never modulo p.
    let sum = |xs: &[u64]| xs.iter().try_fold(0u128, |acc, x| acc.checked_add(u128::from(*x)));
    let into = sum(&[v0, v1]);
    if into.is_none() || into != sum(&[o0, o1, it.public_amount, it.fee]) {
        return Err(IntentRefusal::Unbalanced);
    }
    let names = |a: &[u8; 20]| *a != [0u8; 20];
    let no_word = !pool.names_fee_recipient() && names(&it.fee_recipient);
    if no_word || !representable(&it.recipient, pool) {
        return Err(IntentRefusal::NotRepresentable);
    }
    if names(&it.fee_recipient) && it.fee == 0 {
        return Err(IntentRefusal::FeeRecipientWithoutFee);
    }
    if pool.names_fee_recipient() && it.fee != 0 && !names(&it.fee_recipient) {
        return Err(IntentRefusal::FeeWithoutFeeRecipient);
    }
    if it.public_amount == 0 {
        if names(&it.recipient) || it.clearing_price != 0 {
            return Err(IntentRefusal::TransferHasPublicLeg);
        }
        return (it.fee <= pool.max_relay_fee).then_some(()).ok_or(IntentRefusal::FeeExceedsCap);
    }
    if !fee_within_cap(it.fee, it.public_amount) {
        return Err(IntentRefusal::FeeExceedsCap);
    }
    names(&it.recipient).then_some(()).ok_or(IntentRefusal::RecipientRequired)
}
