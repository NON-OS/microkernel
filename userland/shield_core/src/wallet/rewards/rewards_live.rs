// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::arithmetic_side_effects, clippy::integer_division)]

use super::rewards_vectors::{address, M};
use super::{read_link, EPOCH, GENESIS};
use crate::net::tor::Tor;
use std::time::{SystemTime, UNIX_EPOCH};

/// The registry answers over Tor with the epoch the clock says. Needs the network, so `--ignored`.
#[test]
#[ignore]
fn the_registry_answers_over_tor_in_the_epoch_of_the_clock() {
    let tor = Tor::start(&std::env::temp_dir().join("nox-tor-rewards")).expect("bootstrap");
    let state = read_link(&tor, &address(M), &address(M)).expect("the registry answers");
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("a clock").as_secs();
    assert!(state.started);
    let epoch = (now - GENESIS) / EPOCH;
    assert!(state.epoch == epoch || state.epoch + 1 == epoch, "a block behind the clock at most");
}
