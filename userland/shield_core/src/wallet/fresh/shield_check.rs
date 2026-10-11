//! SHIELD_FROM_FRESH's checks (v1.4.56), before any key is derived: NOX at
//! a fresh address goes into the config's pool as standard deposits only,
//! after an approval of exactly their sum, under the meter's fee caps. What
//! is below the smallest standard size stays at the address.

use nonos_nox_config::{APPROVE_GAS, MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS};

use super::shield_refusal::ShieldRefusal;
use crate::evm::tx::Eip1559;
use crate::wallet::deposit_split::{sizes, MAX_PIECES};

/// The most gas one pool deposit from a fresh address may be given.
pub const SHIELD_DEPOSIT_GAS: u64 = 2_000_000;

/// The standard deposits that fit in `units`, largest first, and the units
/// left over; None when not even the smallest fits, or past MAX_PIECES.
pub fn standard_part(units: u128, allows: impl Fn(u64) -> bool) -> Option<(Vec<u64>, u128)> {
    let mut left = units;
    let mut out = Vec::new();
    for size in sizes(allows) {
        while left >= u128::from(size) && out.len() < MAX_PIECES {
            left = left.checked_sub(u128::from(size))?;
            out.push(size);
        }
    }
    (!out.is_empty()).then_some((out, left))
}

/// Each transaction a fresh address signs to shield: the approval to the
/// token, exactly `approve` when one is due, or a deposit to the pool, none
/// with value.
pub fn shield_checked(
    txs: &[Eip1559],
    (pool, token): ([u8; 20], [u8; 20]),
    approve: Option<&[u8]>,
) -> Result<(), ShieldRefusal> {
    for tx in txs {
        let gas_max = match tx.to {
            t if t == token && approve == Some(&tx.data[..]) => APPROVE_GAS,
            t if t == token => return Err(ShieldRefusal::ApproveNotExact),
            t if t == pool => SHIELD_DEPOSIT_GAS,
            _ => return Err(ShieldRefusal::NotPool),
        };
        let fees =
            tx.max_fee > 0 && tx.max_fee <= MAX_FEE_PER_GAS && tx.max_priority_fee <= tx.max_fee;
        if tx.value != 0 {
            return Err(ShieldRefusal::Value);
        }
        if tx.gas == 0
            || tx.gas > gas_max
            || !fees
            || tx.max_priority_fee > MAX_PRIORITY_FEE_PER_GAS
        {
            return Err(ShieldRefusal::FeesPastCaps);
        }
    }
    Ok(())
}
