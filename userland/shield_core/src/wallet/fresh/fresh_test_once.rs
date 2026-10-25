//! OP_FRESH_ADDRESS mode 1 (v1.4.52): one address per order or channel,
//! past every account and every handed index, kept before it is answered.

use super::super::*;
use super::sign::seed;
use crate::evm::EvmAccount;

fn tmp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("nox-once-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn the_first_index_is_past_the_count_and_every_handed_one() {
    assert_eq!(first_once(3, None), 3);
    assert_eq!(first_once(3, Some(5)), 6);
    assert_eq!(first_once(10, Some(5)), 10);
    assert_eq!(first_once(0, Some(u32::MAX)), u32::MAX, "saturates, never wraps");
}

#[test]
fn two_orders_in_a_row_get_two_addresses() {
    let dir = tmp("two");
    let ahead = derive_ahead(&seed(), first_once(1, None));
    assert_eq!((ahead.len(), ahead[0].0), (AHEAD as usize, 1));
    assert_eq!(ahead[0].1, EvmAccount::at(&seed(), 1).unwrap().address(), "mode 0's address");
    let a = fresh_once(&dir, 1, &ahead).unwrap().unwrap();
    let b = fresh_once(&dir, 1, &ahead).unwrap().unwrap();
    assert_ne!(a, b);
    assert_eq!((handed_index(&dir, &a), handed_index(&dir, &b)), (Some(1), Some(2)));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_added_account_never_makes_a_handed_address_come_back() {
    let ahead = derive_ahead(&seed(), 1);
    assert_eq!(pick_once(2, Some(2), &ahead).map(|p| p.0), Some(3), "count grew to 2");
    assert_eq!(pick_once(9, Some(2), &ahead).map(|p| p.0), Some(9), "count passed them");
    assert_eq!(pick_once(1, Some(AHEAD), &ahead), None, "used up until the next unlock");
    let dir = tmp("bound");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("fresh"), format!("{} {}\n", MAX_HANDED + 1, "ab".repeat(20))).unwrap();
    assert_eq!(highest_handed(&dir), None, "an index past MAX_HANDED is not read");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_scan_batch_never_passes_max_handed() {
    let tail: Vec<u32> = derive_ahead(&seed(), MAX_HANDED - 2).iter().map(|a| a.0).collect();
    assert_eq!(tail, [MAX_HANDED - 2, MAX_HANDED - 1, MAX_HANDED]);
    assert!(derive_ahead(&seed(), MAX_HANDED + 1).is_empty());
    assert!(derive_ahead(&seed(), u32::MAX).is_empty(), "no wrap past the end");
}
