//! Reviews sent as a real mainnet holder named in `NOX_LIVE_HOLDER`: sends and swaps, never signed.

use super::{address, DEAD};
use nox_shield_core::evm::{review, Checked, Network, Order};
use nox_shield_core::net::asset::Coin;
use nox_shield_core::net::tor::Tor;

/// A review sent as a real mainnet holder named in `NOX_LIVE_HOLDER`, read
/// only: a plain transfer, which pays the token's transfer fee, and one into
/// the trading pair, which pays its sell fee. Nothing is signed.
#[test]
#[ignore]
fn a_holders_send_is_worked_out_with_the_tokens_own_fees() {
    let Ok(text) = std::env::var("NOX_LIVE_HOLDER") else {
        println!("set NOX_LIVE_HOLDER to a mainnet holder with ETH and NOX");
        return;
    };
    let from = address(&text);
    let pair = address("0x07ce5889d2eb681af3bd61db24ab2602c502bd1b");
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-account")).expect("bootstrap");
    for (label, to) in [("to a person", DEAD), ("into the pair", pair)] {
        let order = Order { coin: Coin::Nox, to, amount: 1_000_000_000_000_000_000 };
        match review(&tor, Network::Mainnet, &from, &order, &[]).expect("review over tor") {
            Checked::Refused(why) => panic!("{label}: refused: {why}"),
            Checked::Ready(r) => println!(
                "{label}: arrives {} of 10^18, token fee {} bps, gas {}, max network fee {} wei, nonce {}",
                r.arrival.arrives, r.arrival.bps, r.tx.gas, r.max_network_fee, r.tx.nonce
            ),
        }
    }
}
/// Swap reviews as a real mainnet holder named in `NOX_LIVE_HOLDER`, read
/// only: ether into NOX and into USDC, simulated as the holder, and NOX out,
/// which first needs its exact approval. Nothing is signed.
#[test]
#[ignore]
fn a_holders_swaps_are_quoted_simulated_and_approved_exactly() {
    use nox_shield_core::evm::swap::{review_swap, Swap, SwapChecked};
    let Ok(text) = std::env::var("NOX_LIVE_HOLDER") else {
        println!("set NOX_LIVE_HOLDER to a mainnet holder with ETH and NOX");
        return;
    };
    let from = address(&text);
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-account")).expect("bootstrap");
    // The amount of ether to swap, in wei, from `NOX_LIVE_WEI`, or 0.01 ETH.
    let tenth: u128 = std::env::var("NOX_LIVE_WEI")
        .ok()
        .and_then(|w| w.parse().ok())
        .unwrap_or(10_000_000_000_000_000);
    let cases = [
        (Coin::Eth, Coin::Nox, tenth),
        (Coin::Eth, Coin::Usdc, tenth),
        (Coin::Nox, Coin::Eth, 1_000 * 1_000_000_000_000_000_000),
    ];
    for (a, b, amount) in cases {
        let swap = Swap { from: a, to: b, amount, slippage_bps: 50 };
        match review_swap(&tor, Network::Mainnet, &from, &swap, &[]).expect("swap over tor") {
            SwapChecked::Refused(why) => println!("{a:?} to {b:?}: refused: {why}"),
            SwapChecked::Ready(r) => println!(
                "{a:?} to {b:?}: approval first {}, expected {}, minimum {}, token fee {}, impact {} bps, gas {}",
                r.approval, r.expected, r.minimum, r.token_fee, r.impact_bps, r.tx.gas
            ),
        }
    }
}
