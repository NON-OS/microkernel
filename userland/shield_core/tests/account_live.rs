//! The public account against both real networks, over the wallet's own Tor.
//!
//! Reads only: balances, and a review of a send that is never signed. The
//! account is the public BIP-39 test account, so nothing here is anyone's.
//! Ignored by default because it needs the network. Run it with `--ignored`.
#![allow(clippy::expect_used, clippy::panic)]

use nox_shield_core::evm::{balances, review, Checked, Network, Order};
use nox_shield_core::net::asset::Coin;
use nox_shield_core::net::tor::Tor;
use std::time::Instant;

/// 0x9858EfFD232B4033E47d90003D41EC34EcaEda94, "abandon ... about".
const TEST: [u8; 20] = [
    0x98, 0x58, 0xef, 0xfd, 0x23, 0x2b, 0x40, 0x33, 0xe4, 0x7d, 0x90, 0x00, 0x3d, 0x41, 0xec, 0x34,
    0xec, 0xae, 0xda, 0x94,
];
const DEAD: [u8; 20] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xde, 0xad];

#[test]
#[ignore]
fn both_networks_answer_over_tor_and_a_nox_send_is_worked_out() {
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-account")).expect("bootstrap");
    for network in [Network::Mainnet, Network::Sepolia] {
        let t = Instant::now();
        let held = balances(&tor, network, &TEST).expect("balances over tor");
        println!(
            "{network:?}: {} wei, {} NOX units, {} USDC units, in {:?}",
            held.eth,
            held.nox,
            held.usdc,
            t.elapsed()
        );
        let order = Order { coin: Coin::Nox, to: DEAD, amount: 1_000_000_000_000_000 };
        let t = Instant::now();
        match review(&tor, network, &TEST, &order, &[]).expect("review over tor") {
            Checked::Refused(why) => {
                println!("{network:?}: refused in {:?}: {why}", t.elapsed());
                assert!(
                    !why.contains("changed"),
                    "the token's implementation is not the pinned one"
                );
            }
            Checked::Ready(r) => println!(
                "{network:?}: ready in {:?}: arrives {} gas {} max fee {} wei",
                t.elapsed(),
                r.arrival.arrives,
                r.tx.gas,
                r.max_network_fee
            ),
        }
    }
}

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

fn address(text: &str) -> [u8; 20] {
    let digits = text.trim().trim_start_matches("0x");
    let mut out = [0u8; 20];
    for (byte, pair) in out.iter_mut().zip(digits.as_bytes().chunks_exact(2)) {
        let pair = std::str::from_utf8(pair).expect("ascii");
        *byte = u8::from_str_radix(pair, 16).expect("hex");
    }
    out
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
