//! The scanner against the real pool, over the wallet's own Tor.
//!
//! No proxy app: the core bootstraps from the live consensus, builds its own
//! circuit, wraps TLS end to end and reads the Sepolia pool's history. It needs the
//! network, so it is ignored by default. Run it with `--ignored`.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic, clippy::unwrap_used)]

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

#[test]
#[ignore]
fn the_pre_flight_reads_the_live_pools_over_tor_and_refuses_correctly() {
    use nox_shield_core::net::pool::{Pool, ACTIVE, FORMAT5, NOX_ASSET, RPCS};
    use nox_shield_core::wallet::{check_deposit, read_pool_state_checked, Refusal};
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-live")).expect("bootstrap");
    // An address nobody put on any list. The live pool is out of beta mode, so it may deposit.
    let stranger = [0x42u8; 20];

    let live =
        read_pool_state_checked(&tor, &RPCS, &ACTIVE, &stranger, NOX_ASSET).expect("format 5 pool");
    println!("format 5 pool: {live:?}");
    assert!(!live.wound_down, "the format 5 pool is the live one");
    assert_eq!(check_deposit(&live, 1_000_000_000_000_000_000), Ok(()));

    let wound = Pool { address: "0x4F112D4879c61aB8082492f4447E846812E45c53", ..FORMAT5 };
    let trap =
        read_pool_state_checked(&tor, &RPCS, &wound, &stranger, NOX_ASSET).expect("trap pool");
    println!("trap pool: {trap:?}");
    assert!(trap.wound_down && !trap.deposits_paused, "wound down, yet still taking deposits");
    assert_eq!(check_deposit(&trap, 1_000_000_000_000_000_000), Err(Refusal::WoundDown));
}

#[test]
#[ignore]
fn what_a_user_could_recover_is_read_over_tor() {
    use nox_shield_core::net::pool::{NOX_ASSET, RPCS};
    use nox_shield_core::wallet::read_recoverable;
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-live")).expect("bootstrap");
    // Nothing was ever credited to or deposited by this address.
    let got = read_recoverable(
        &tor,
        &RPCS,
        "0x4F112D4879c61aB8082492f4447E846812E45c53",
        &[0x42; 20],
        NOX_ASSET,
    )
    .expect("read");
    println!("claimable, refundable: {got:?}");
    assert_eq!(got, (0, 0));
}

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
