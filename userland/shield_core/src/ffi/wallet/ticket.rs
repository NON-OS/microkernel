//! What a deposit hands the screen: the refusal in a sentence, or the two
//! transactions in order, with every amount already formatted.

#[path = "ticket_sentence.rs"]
mod sentence;

pub(super) use sentence::sentence;

use crate::error::WalletError;
use crate::ffi::chain_types::{format_wide, DepositTicket};
use crate::ffi::evm::{parse_evm_address, to_hex};
use crate::net::asset::Asset;
use crate::net::pool::ACTIVE;
use crate::wallet::DepositRequest;

/// `approve(address,uint256)`.
const APPROVE: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];

/// The ticket for a deposit the pool would take. ETH goes as the deposit's
/// value and needs no approval, so its approval fields are empty.
pub(super) fn ticket(
    request: &DepositRequest,
    asset: &Asset,
) -> Result<DepositTicket, WalletError> {
    let note =
        u128::from(request.note.value).checked_mul(asset.scale).ok_or(WalletError::Amount)?;
    let approve_data = match asset.token {
        Some(_) => to_hex(&approve_calldata(request.amount)?),
        None => String::new(),
    };
    Ok(DepositTicket {
        refusal: None,
        amount: format_wide(request.amount),
        fee: format_wide(request.fee),
        note: format_wide(note),
        approve_to: asset.token.unwrap_or_default().to_string(),
        approve_data,
        deposit_to: ACTIVE.address.to_string(),
        deposit_data: to_hex(&request.calldata),
        value_wei: request.value_wei.to_string(),
    })
}

/// `approve(pool, amount)` on the token, so the pool can pull the deposit.
pub(crate) fn approve_calldata(amount: u128) -> Result<Vec<u8>, WalletError> {
    let pool = parse_evm_address(ACTIVE.address)?;
    let mut out = APPROVE.to_vec();
    out.extend_from_slice(&[0u8; 12]);
    out.extend_from_slice(&pool);
    out.extend_from_slice(&[0u8; 16]);
    out.extend_from_slice(&amount.to_be_bytes());
    Ok(out)
}

/// A deposit that was not kept, and why.
pub(super) fn refused(amount: u128, why: &str) -> DepositTicket {
    DepositTicket {
        refusal: Some(why.to_string()),
        amount: format_wide(amount),
        fee: String::new(),
        note: String::new(),
        approve_to: String::new(),
        approve_data: String::new(),
        deposit_to: String::new(),
        deposit_data: String::new(),
        value_wei: String::new(),
    }
}
