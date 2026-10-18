//! A deposit's reads of the live pools: the pre-flight, and what a user could recover.

use nox_shield_core::net::tor::Tor;

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
