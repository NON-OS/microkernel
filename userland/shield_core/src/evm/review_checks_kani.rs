//! Proved for every balance, amount and fee.

use super::afford;
use crate::evm::review_parts::Order;
use crate::net::asset::Coin;

/// A send that passes is always covered. Ether sends keep the amount and the
/// most the network can charge, and token sends keep the network fee in ether.
#[kani::proof]
fn a_send_that_passes_the_balance_check_is_always_covered() {
    let (ether, amount, max_fee): (u128, u128, u128) = (kani::any(), kani::any(), kani::any());
    let coin = match kani::any::<u8>() % 3 {
        0 => Coin::Eth,
        1 => Coin::Nox,
        _ => Coin::Usdc,
    };
    let order = Order { coin, to: [0u8; 20], amount };
    if afford(&order, ether, max_fee).is_none() {
        match coin {
            Coin::Eth => assert!(amount.checked_add(max_fee).is_some_and(|n| n <= ether)),
            Coin::Nox | Coin::Usdc => assert!(max_fee <= ether),
        }
    }
}
