//! Several deposits of one typed amount as a screen shows them, every amount in whole coins.

use crate::evm::units::ether;
use crate::ffi::chain_types::format_wide;
use crate::wallet::DepositRequest;

/// One standard deposit of the amount, its pool fee, and what its note holds after the fee.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShieldPiece {
    pub amount: String,
    pub pool_fee: String,
    pub shielded: String,
}

/// The deposits of one amount under one review, or the exact approval of the whole that comes
/// first, which `confirm_public_send` confirms.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ShieldSplit {
    /// The review to confirm, or 0 with a refusal.
    pub id: u64,
    pub refusal: Option<String>,
    pub approval: bool,
    pub pieces: Vec<ShieldPiece>,
    pub amount: String,
    pub pool_fee: String,
    pub shielded: String,
    pub max_network_fee: String,
    pub valid_for_seconds: u32,
}

pub(super) fn plan(id: u64, requests: &[DepositRequest], scale: u128, fee: u128) -> ShieldSplit {
    let held = |r: &DepositRequest| u128::from(r.note.value).saturating_mul(scale);
    let sum = |f: &dyn Fn(&DepositRequest) -> u128| {
        requests.iter().fold(0u128, |total, r| total.saturating_add(f(r)))
    };
    ShieldSplit {
        id,
        refusal: None,
        approval: false,
        pieces: requests
            .iter()
            .map(|r| ShieldPiece {
                amount: format_wide(r.amount),
                pool_fee: format_wide(r.fee),
                shielded: format_wide(held(r)),
            })
            .collect(),
        amount: format_wide(sum(&|r| r.amount)),
        pool_fee: format_wide(sum(&|r| r.fee)),
        shielded: format_wide(sum(&held)),
        max_network_fee: ether(fee),
        valid_for_seconds: u32::try_from(super::super::account_view::VALID_FOR.as_secs())
            .unwrap_or(0),
    }
}

pub(super) fn refused(why: &str) -> ShieldSplit {
    ShieldSplit { refusal: Some(why.to_string()), ..ShieldSplit::default() }
}

#[cfg(test)]
#[path = "view_test.rs"]
mod view_test;
