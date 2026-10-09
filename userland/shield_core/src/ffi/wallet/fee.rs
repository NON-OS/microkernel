//! The least fee of a private transfer, at the lowest rung. The fee now is `quote_spend`.

use crate::net::asset::Coin;
use crate::net::pool::ACTIVE;

pub fn relay_fee(coin: Coin) -> String {
    let fee = ACTIVE.asset(coin).map_or(0, |a| u128::from(a.relay_fee).saturating_mul(a.scale));
    crate::evm::units::format(fee, coin.decimals())
}
