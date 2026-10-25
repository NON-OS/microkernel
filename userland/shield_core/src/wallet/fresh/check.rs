//! SIGN_FROM_FRESH's checks, before any key is derived (v1.4.51): the
//! config's Escrow on that chain, no value, refund or dispute alone, a
//! refund paying the address that signs, and the meter's fee caps.

use nonos_nox_config::{DISPUTE_GAS, MAX_FEE_PER_GAS, MAX_PRIORITY_FEE_PER_GAS, REFUND_GAS};

use super::request::{FreshRefusal, FreshSign};
use crate::evm::tx::Eip1559;
use FreshRefusal::*;

/// dispute(T,uint64,bytes,bytes32,bytes32[],bytes32[]) and refund(T,address,bytes32).
pub const DISPUTE: [u8; 4] = [0xd8, 0x4d, 0x6e, 0xf9];
pub const REFUND: [u8; 4] = [0xa3, 0x81, 0x4a, 0x14];
/// refund's refundTo word, after the selector and T's 14 inline words.
const REFUND_TO_AT: usize = 4 + 14 * 32;

fn word_of(address: &[u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w.iter_mut().skip(12).zip(address).for_each(|(o, a)| *o = *a);
    w
}

/// The kind's gas limit, once its selector, and a refund's payee, are right.
fn gas_limit(r: &FreshSign) -> Result<u64, FreshRefusal> {
    match r.data.get(..4) {
        Some(s) if s == DISPUTE => Ok(DISPUTE_GAS),
        Some(s) if s == REFUND => {
            let to = r.data.get(REFUND_TO_AT..REFUND_TO_AT.saturating_add(32));
            match to == Some(&word_of(&r.address)[..]) {
                true => Ok(REFUND_GAS),
                false => Err(OtherRefundTo),
            }
        }
        _ => Err(OtherSelector),
    }
}

/// The transaction to sign with account `handed`'s key, or why not.
/// `escrow` is the config's Escrow on a chain, None where there is none.
pub fn checked(
    r: &FreshSign,
    handed: Option<u32>,
    escrow: impl Fn(u64) -> Option<[u8; 20]>,
) -> Result<(u32, Eip1559), FreshRefusal> {
    let index = handed.ok_or(UnknownAddress)?;
    let to = escrow(r.chain_id).ok_or(NotConfigChain)?;
    if r.to != to {
        return Err(OtherTo);
    }
    if r.value != 0 {
        return Err(Value);
    }
    let most = gas_limit(r)?;
    let fees = r.max_fee > 0 && r.max_fee <= MAX_FEE_PER_GAS && r.max_priority <= r.max_fee;
    if r.gas == 0 || r.gas > most || !fees || r.max_priority > MAX_PRIORITY_FEE_PER_GAS {
        return Err(FeesPastCaps);
    }
    let tx = Eip1559 {
        chain_id: r.chain_id,
        nonce: r.nonce,
        max_priority_fee: r.max_priority,
        max_fee: r.max_fee,
        gas: r.gas,
        to,
        value: 0,
        data: r.data.clone(),
    };
    Ok((index, tx))
}
