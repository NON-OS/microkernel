//! The public account against both real networks, over the wallet's own Tor.
//!
//! Reads only: balances, and a review of a send that is never signed. The
//! account is the public BIP-39 test account, so nothing here is anyone's.
//! Ignored by default because it needs the network. Run it with `--ignored`.

#[path = "account_live/holder.rs"]
mod holder;

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

fn address(text: &str) -> [u8; 20] {
    let digits = text.trim().trim_start_matches("0x");
    let mut out = [0u8; 20];
    for (byte, pair) in out.iter_mut().zip(digits.as_bytes().chunks_exact(2)) {
        let pair = std::str::from_utf8(pair).expect("ascii");
        *byte = u8::from_str_radix(pair, 16).expect("hex");
    }
    out
}
