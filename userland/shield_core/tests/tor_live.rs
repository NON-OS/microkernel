//! The scanner against the real pool, over the wallet's own Tor.
//!
//! No proxy app: the core bootstraps from the live consensus, builds its own
//! circuit, wraps TLS end to end and reads the Sepolia pool's history. It needs the
//! network, so it is ignored by default. Run it with `--ignored`.

#[path = "tor_live/deposit.rs"]
mod deposit;
#[path = "tor_live/sync.rs"]
mod sync;

use nox_shield_core::discovery::{first_gap, note_commitments, Log};
use nox_shield_core::net::rpc::head_over_tor;
use nox_shield_core::net::tor::Tor;
use std::time::Instant;

const HOST: &str = "ethereum-sepolia-rpc.publicnode.com";

#[test]
#[ignore]
fn the_scanner_reads_the_real_pool_over_the_wallets_own_tor() {
    use nox_shield_core::net::pool::RPCS;
    use nox_shield_core::wallet::fetch_history;
    let t = Instant::now();
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-live")).expect("bootstrap");
    println!("bootstrapped in {:?}", t.elapsed());

    let t = Instant::now();
    let head = head_over_tor(&tor, HOST).expect("head over tor");
    println!("chain head {head} in {:?}", t.elapsed());
    assert!(head > 11_764_494);

    // The history is accepted only whole: every leaf the pool counts, with no hole. A server that
    // serves less is refused and the next one asked, which is what the wallet itself does.
    let t = Instant::now();
    let history = fetch_history(&tor, &RPCS).expect("a whole history from one of the pool RPCs");
    let logs: Vec<Log> =
        history.committed.iter().map(|r| Log { topics: &r.topics, data: &r.data }).collect();
    let leaves = note_commitments(&logs);
    assert!(first_gap(&leaves).is_none() && !leaves.is_empty());
    println!("{} leaves, whole, in {:?} over Tor", leaves.len(), t.elapsed());
    let registered = history.registered.as_ref().expect("the v2 pool has a registry");
    assert!(!registered.is_empty(), "at least the current root is registered");
    println!("{} of the newest roots are registered", registered.len());

    // A read after the first goes on from the history kept, and is the same history.
    use nox_shield_core::net::rpc::progress::Stop;
    use nox_shield_core::wallet::fetch_history_until;
    let t = Instant::now();
    let on = fetch_history_until(&tor, &RPCS, Stop::never(), Some(&history)).expect("read on");
    println!("read on from block {} in {:?} over Tor", history.head, t.elapsed());
    assert!(on.head >= history.head && on.committed.len() >= history.committed.len());
    let blocks = |l: &[nox_shield_core::net::rpc::RawLog]| -> Vec<u64> {
        l.iter().map(|r| r.block).collect()
    };
    let n = history.committed.len().min(on.committed.len());
    assert_eq!(blocks(&on.committed[..n]), blocks(&history.committed[..n]));
}
