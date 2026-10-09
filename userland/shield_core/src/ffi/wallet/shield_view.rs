//! A deposit the pool or the chain would refuse, as the screen shows it.

use super::account_view::VALID_FOR;
use crate::evm::shield::Shielding;
use crate::ffi::chain_types::format_wide;
use crate::ffi::PublicShield;

pub(super) fn refused(amount: u128, why: &str) -> PublicShield {
    PublicShield {
        id: 0,
        refusal: Some(why.to_string()),
        approval: false,
        amount: format_wide(amount),
        pool_fee: String::new(),
        shielded: String::new(),
        max_network_fee: String::new(),
        valid_for_seconds: 0,
    }
}

/// A review ready to confirm. `amounts` are what is sent, the pool fee and what is shielded.
pub(super) fn shown(id: u64, review: &Shielding, amounts: [u128; 3]) -> PublicShield {
    let [amount, fee, shielded] = amounts;
    PublicShield {
        id,
        refusal: None,
        approval: review.approval,
        amount: format_wide(amount),
        pool_fee: format_wide(fee),
        shielded: format_wide(shielded),
        max_network_fee: crate::evm::units::ether(review.max_network_fee),
        valid_for_seconds: u32::try_from(VALID_FOR.as_secs()).unwrap_or(0),
    }
}
