// NONOS Operating System (AGPL-3.0-or-later)

//! The background scanner: a running, deduplicated picture of the networks in
//! range that the serve loop fills a little at a time between requests, so a scan
//! request is answered instantly from the cache instead of blocking the caller for
//! a full channel sweep. It only advances while the radio is up and not
//! associated, so hopping channels never disturbs a live connection.

use crate::scan;

use super::channels::SCAN_FRAME_MAX;

pub(in super::super) struct Scanner {
    pub(super) results: scan::ScanResults,
    pub(super) frame: [u8; SCAN_FRAME_MAX],
    pub(super) ch_idx: usize,
    /// Uptime at which the scan moves to the next channel.
    pub(super) hop_at_ms: i64,
    pub(super) tuned: bool,
    // Counters, so a serial-less boot can see where the receive path stops: how
    // many passes ran, how many raw frames the ring delivered, and how many of
    // those parsed as beacons.
    pub(in super::super) steps: u32,
    pub(in super::super) raw: u32,
    pub(in super::super) beacons: u32,
    pub(super) sweeps: u32,
}

impl Scanner {
    pub(in super::super) fn new() -> Self {
        Self {
            results: scan::ScanResults::new(),
            frame: [0u8; SCAN_FRAME_MAX],
            ch_idx: 0,
            hop_at_ms: 0,
            tuned: false,
            steps: 0,
            raw: 0,
            beacons: 0,
            sweeps: 0,
        }
    }

    pub(in super::super) fn cache(&self) -> &scan::ScanResults {
        &self.results
    }
}
