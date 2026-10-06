/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use super::{record, recorded, KEPT};
use crate::evm::Network;

#[test]
fn sends_are_kept_per_network_and_account_newest_last() {
    let dir = std::env::temp_dir().join(format!("sends-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let (me, other) = ([1u8; 20], [2u8; 20]);
    assert!(recorded(&dir, Network::Mainnet, &me).is_empty());
    record(&dir, Network::Mainnet, &me, 4, &[4; 32]).unwrap();
    record(&dir, Network::Mainnet, &me, 2, &[2; 32]).unwrap();
    assert_eq!(recorded(&dir, Network::Mainnet, &me), vec![(2, [2; 32]), (4, [4; 32])]);
    assert!(recorded(&dir, Network::Sepolia, &me).is_empty(), "the other network");
    assert!(recorded(&dir, Network::Mainnet, &other).is_empty(), "another account");
    for n in 10..40 {
        record(&dir, Network::Mainnet, &me, n, &[7; 32]).unwrap();
    }
    let kept = recorded(&dir, Network::Mainnet, &me);
    assert_eq!(kept.len(), KEPT);
    assert_eq!(kept.last().unwrap().0, 39, "the newest are the ones kept");
    let _ = std::fs::remove_dir_all(&dir);
}
