//! A fresh wallet syncs the live format 5 pool whole, and owns nothing on it.

use nox_shield_core::net::tor::Tor;
use std::time::Instant;

#[test]
#[ignore]
fn a_wallet_syncs_the_live_format_5_pool_whole_over_tor() {
    use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
    use nox_shield_core::keys::Account;
    use nox_shield_core::net::pool::RPCS;
    use nox_shield_core::wallet::{fetch_history, sync_chain};
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-live")).expect("bootstrap");
    let t = Instant::now();
    let h = fetch_history(&tor, &RPCS).expect("a whole history from some RPC");
    println!(
        "head {}: {} leaves, {} outputs, {} nullifiers in {:?}",
        h.head,
        h.committed.len(),
        h.outputs.len(),
        h.nullifiers.len(),
        t.elapsed()
    );
    let me =
        Account::from_seed(&phrase_to_seed(&generate_phrase().expect("e")).expect("s")).expect("a");
    let (_, scan) = sync_chain(&tor, &RPCS, &me, &[], &[]).expect("sync");
    assert!(
        scan.received.is_empty() && scan.deposited.is_empty(),
        "a fresh wallet owns nothing on the pool"
    );
}
