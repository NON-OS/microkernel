//! How far the scan of the pool's history has come, for the wallet to say while a sync runs:
//! which of the histories is being read, and the last block read of it. Counters only, never a
//! log or an address.

#[path = "progress_stop.rs"]
mod stop;

pub use stop::Stop;

use std::sync::Mutex;

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

#[cfg(test)]
#[path = "progress_test.rs"]
mod tests;
