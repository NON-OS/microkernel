//! The scan's progress and its stop, as a sync and a lock see them.

use super::*;
use crate::error::NetError;
use std::sync::atomic::{AtomicU64, Ordering};

#[test]
fn a_failed_scan_keeps_where_it_stopped_and_a_page_moves_the_block() {
    at("sepolia.gateway.tenderly.co", "the deposits' history");
    set(Some(ScanProgress { history: 1, histories: 4, block: 10, head: 100, from: 10 }));
    read_to(55);
    assert_eq!(scan_progress().map(|p| p.block), Some(55));
    set(None);
    assert_eq!(scan_progress(), None, "a finished or failed scan reports no progress");
    assert_eq!(
        last_step(),
        Some(("sepolia.gateway.tenderly.co", "the deposits' history")),
        "but still names the server and step it stopped at"
    );
}

#[test]
fn a_read_stops_once_told_to_and_a_later_one_runs() {
    let (mine, other) = (AtomicU64::new(0), AtomicU64::new(0));
    let read = Stop::watching(&mine);
    let elsewhere = Stop::watching(&other);
    assert_eq!(read.check(), Ok(()));
    mine.fetch_add(1, Ordering::SeqCst);
    assert_eq!(read.check(), Err(NetError::Stopped));
    assert_eq!(elsewhere.check(), Ok(()), "another wallet's read runs on");
    assert_eq!(Stop::watching(&mine).check(), Ok(()), "a read started after the stop runs");
    assert_eq!(Stop::never().check(), Ok(()));
}
