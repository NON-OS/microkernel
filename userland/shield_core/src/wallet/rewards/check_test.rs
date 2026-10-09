// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::arithmetic_side_effects)]

use super::{counts_from, may_link, may_unlink, LinkRefusal, LinkState, EPOCH, GENESIS};

const M: [u8; 20] = [1; 20];
const T: [u8; 20] = [2; 20];
const OTHER: [u8; 20] = [3; 20];

fn state(linked_to: Option<[u8; 20]>, testnet: Option<[u8; 20]>, nonce: u128) -> LinkState {
    LinkState {
        linked_to,
        testnet,
        from_epoch: 1,
        changed_in: 4,
        contract_wallet: false,
        nonce,
        started: true,
        epoch: 4,
    }
}

/// Each refusal the registry would revert with is named first, from what it holds.
#[test]
fn a_link_the_registry_would_refuse_is_refused_here() {
    assert_eq!(may_link(&state(None, None, 0), &M, &T), Ok(()));
    let taken = may_link(&state(Some(OTHER), None, 0), &M, &T);
    assert_eq!(taken, Err(LinkRefusal::TestnetTaken));
    let same = may_link(&state(Some(M), Some(T), 1), &M, &T);
    assert_eq!(same, Err(LinkRefusal::AlreadyLinked), "a second link moves its first epoch");
    let moved = may_link(&state(None, Some(OTHER), 1), &M, &T);
    assert_eq!(moved, Err(LinkRefusal::ChangedThisEpoch));
    let before = LinkState { started: false, ..state(None, Some(OTHER), 1) };
    assert_eq!(may_link(&before, &M, &T), Ok(()), "before genesis a link may change freely");
    assert_eq!(may_unlink(&state(None, None, 0)), Err(LinkRefusal::NotLinked));
    assert_eq!(may_unlink(&state(Some(M), Some(T), 1)), Ok(()));
}

#[test]
fn a_link_counts_from_the_next_epoch_or_from_genesis() {
    assert_eq!(counts_from(&state(None, None, 0)), (5, GENESIS + 5 * EPOCH));
    let before = LinkState { started: false, ..state(None, None, 0) };
    assert_eq!(counts_from(&before), (0, GENESIS));
}
