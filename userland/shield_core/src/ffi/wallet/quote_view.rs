//! A quoted fee as a screen shows it: each part in whole coins, the rung, and the base fee.

use crate::evm::units::{format, gwei};
use crate::net::asset::Asset;
use crate::net::fee_quote::Quote;

/// A fee as a screen shows it, in whole coins, with the refusal when the policy would refuse it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpendQuote {
    pub network_fee: String,
    pub protocol_fee: String,
    pub total_fee: String,
    /// The rung of the gas ladder, 1 to 4, and the base fee when the fee was quoted.
    pub rung: u8,
    pub base_fee_gwei: String,
    pub refusal: Option<String>,
}

pub(super) fn shown(q: &Quote, asset: Asset) -> SpendQuote {
    let coins = |units: u64| format(u128::from(units).saturating_mul(asset.scale), 18);
    SpendQuote {
        network_fee: coins(q.network),
        protocol_fee: coins(q.protocol),
        total_fee: coins(q.protocol.saturating_add(q.network)),
        rung: u8::try_from(q.rung.saturating_add(1)).unwrap_or(u8::MAX),
        base_fee_gwei: gwei(q.base_fee),
        refusal: None,
    }
}

pub(super) fn refused() -> SpendQuote {
    let why = "The pool would not take this spend at any fee now. Nothing was proved.";
    SpendQuote {
        network_fee: String::new(),
        protocol_fee: String::new(),
        total_fee: String::new(),
        rung: 0,
        base_fee_gwei: String::new(),
        refusal: Some(why.to_string()),
    }
}
