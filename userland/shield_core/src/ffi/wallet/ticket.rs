//! What a deposit hands the screen: the refusal in a sentence, or the two
//! transactions in order, with every amount already formatted.

use crate::error::WalletError;
use crate::ffi::chain_types::{format_wide, DepositTicket};
use crate::ffi::evm::{parse_evm_address, to_hex};
use crate::net::asset::Asset;
use crate::net::pool::ACTIVE;
use crate::wallet::{DepositRequest, Refusal};

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

/// The pool's refusal, the way a screen says it.
pub(super) fn sentence(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::DepositsPaused => "The pool has paused deposits.",
        Refusal::WoundDown => "This pool is wound down. Money sent to it could not be spent.",
        Refusal::Amount => "That amount is zero, too large for one note, or all fee.",
        Refusal::NotWholeUnits => "The pool takes whole units only. Round the amount.",
        Refusal::BetaPaused => "The beta is paused.",
        Refusal::NotOnBetaList => "That EVM address is not on the beta list.",
        Refusal::AddressCapReached => "That EVM address has reached its beta limit.",
        Refusal::PoolCapReached => "The pool has reached its beta limit.",
    }
}
