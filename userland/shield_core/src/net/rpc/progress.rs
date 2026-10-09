//! How far the scan of the pool's history has come, for the wallet to say while a sync runs:
//! which of the histories is being read, and the last block read of it. Counters only, never a
//! log or an address.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use crate::error::NetError;

/// One look at a running scan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScanProgress {
    /// The history being read, from 1, of `histories`.
    pub history: u32,
    pub histories: u32,
    /// The last block read of it, and the block the scan reads to.
    pub block: u64,
    pub head: u64,
    /// The first block of the scan.
    pub from: u64,
}

static NOW: Mutex<Option<ScanProgress>> = Mutex::new(None);

pub(crate) fn set(p: Option<ScanProgress>) {
    if let Ok(mut now) = NOW.lock() {
        *now = p;
    }
}

pub(crate) fn read_to(block: u64) {
    if let Ok(mut now) = NOW.lock() {
        if let Some(p) = now.as_mut() {
            p.block = block;
        }
    }
}

/// The scan running now, if one is.
pub fn scan_progress() -> Option<ScanProgress> {
    NOW.lock().ok().and_then(|p| *p)
}

/// Where the scan is: the server it reads and what it asks of it. Kept after a failure, so the
/// failure can be said with the server and the step it failed at.
static AT: Mutex<Option<(&'static str, &'static str)>> = Mutex::new(None);

/// The scan asks `host` for `step`.
pub(crate) fn at(host: &'static str, step: &'static str) {
    if let Ok(mut at) = AT.lock() {
        *at = Some((host, step));
    }
}

/// The server and the step the scan last asked for: where a failed scan stopped.
pub fn last_step() -> Option<(&'static str, &'static str)> {
    AT.lock().ok().and_then(|a| *a)
}

/// What a read of the pool watches to know it should stop: a count of the stops its wallet
/// was told, and that count when the read started. A read stops at its next request once the
/// count has moved, so a lock never waits minutes on a sync over a mixnet, and one wallet's
/// lock never stops another's read.
#[derive(Clone, Copy)]
pub struct Stop<'a> {
    stops: &'a AtomicU64,
    epoch: u64,
}

/// The count of a read that is never stopped.
static NEVER: AtomicU64 = AtomicU64::new(0);

impl<'a> Stop<'a> {
    /// A read stopped by any later stop counted in `stops`.
    pub fn watching(stops: &'a AtomicU64) -> Stop<'a> {
        Stop { stops, epoch: stops.load(Ordering::SeqCst) }
    }

    /// A read nothing stops, for the callers that hold no wallet.
    pub fn never() -> Stop<'static> {
        Stop::watching(&NEVER)
    }

    /// Whether this read was told to stop since it started.
    pub(crate) fn check(&self) -> Result<(), NetError> {
        if self.stops.load(Ordering::SeqCst) == self.epoch {
            Ok(())
        } else {
            Err(NetError::Stopped)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
