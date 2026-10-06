//! A deposit via `absorb`, kept at its net value after the pool's rounded-down fee.
//! Sent from the user's own EVM wallet, never relayed, or a refund would go to the relayer.

use super::abi::word;
use super::deposit_gate::units_of;
use crate::error::WalletError;
use crate::keys::Address;
use crate::notes::{commitment, fresh_blinding, owner_commit_wire, wire_digest, NotePlaintext};
use crate::prover::pool_hasher;

const ABSORB: [u8; 4] = [0x5c, 0xb9, 0x45, 0xfa];
pub const NATIVE_ASSET: u64 = 0;
const BPS: u128 = 10_000;

/// A deposit: the call the user's wallet signs, and the note kept here, the only opening record.
pub struct DepositRequest {
    /// The note at its net value, in pool units. Secret, the blinding is in it.
    pub note: NotePlaintext,
    /// What the depositor sends, and the pool's fee, both in base units.
    pub amount: u128,
    pub fee: u128,
    /// `ownerCommit`, packed as the pool reads it.
    pub owner_commit: [u8; 32],
    /// The leaf the pool will store, to recognise this deposit in its logs.
    pub commitment: [u8; 32],
    /// The transaction's data and value, for the user's own EVM wallet.
    pub calldata: Vec<u8>,
    pub value_wei: u128,
}

/// The pool's split into fee and note value, in units. Nothing if the fee takes it all.
pub fn split_deposit(amount: u64, fee_bps: u16) -> Option<(u64, u64)> {
    let fee = u128::from(amount).checked_mul(u128::from(fee_bps))?.checked_div(BPS)?;
    let fee = u64::try_from(fee).ok()?;
    let value = amount.checked_sub(fee)?;
    (value > 0).then_some((fee, value))
}

/// Build a deposit of `amount` base units, holding `amount / scale - fee` whole note units.
pub fn prepare_deposit(
    to: &Address,
    amount: u128,
    asset_id: u64,
    fee_bps: u16,
    scale: u128,
) -> Result<DepositRequest, WalletError> {
    let units = units_of(amount, scale).map_err(|_| WalletError::Amount)?;
    let (fee_units, value) = split_deposit(units, fee_bps).ok_or(WalletError::Amount)?;
    let fee = u128::from(fee_units).checked_mul(scale).ok_or(WalletError::Amount)?;
    let note =
        NotePlaintext { value, asset_id, blinding: fresh_blinding()?, spend_pk: to.spend_pk };
    let cm = commitment(&pool_hasher(), &note.note());
    let owner_commit = owner_commit_wire(&note.note());
    let calldata = absorb_calldata(asset_id, amount, &owner_commit);
    Ok(DepositRequest {
        amount,
        fee,
        owner_commit,
        commitment: wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()]),
        calldata,
        value_wei: if asset_id == NATIVE_ASSET { amount } else { 0 },
        note,
    })
}

pub fn absorb_calldata(asset_id: u64, amount: u128, owner_commit: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 96);
    out.extend_from_slice(&ABSORB);
    out.extend_from_slice(&word(u128::from(asset_id)));
    out.extend_from_slice(&word(amount));
    out.extend_from_slice(owner_commit);
    out
}
