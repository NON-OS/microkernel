/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The scan op: the networks the driver has heard.
//!
//! The RTL8821CE answers from its background scanner's running picture, so the
//! reply is quick; the timeout covers a driver that sweeps on request. The
//! reply is three little-endian counters (scan passes, raw frames, beacons
//! parsed) and then the network list `wire::parse_scan` reads.

use super::call::{word, OP_SCAN, WIFI_HDR};
use super::find::Driver;
use crate::network::ScanNetwork;
use crate::wire::parse_scan;

const SCAN_TIMEOUT_MS: u64 = 15_000;
/// The count byte and the longest network list the driver sends.
const RESP_MAX: usize = 1024;
const STATS_LEN: usize = 12;

/// What a scan attempt resolved to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScanOutcome {
    /// No Wi-Fi driver service is registered.
    NoService,
    /// The driver did not answer with a scan reply.
    NoResponse,
    /// The driver answered; the list, possibly empty, is complete.
    Scanned,
}

/// The driver's receive-pipeline counters from the scan reply, so a panel can
/// show where reception stops when no networks come back.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct ScanStats {
    pub steps: u32,
    pub raw: u32,
    pub beacons: u32,
}

impl Driver {
    /// Fill `out` with up to its length of networks. Returns how many were
    /// written, how the attempt resolved, and the counters.
    pub fn scan(&self, out: &mut [ScanNetwork]) -> (usize, ScanOutcome, ScanStats) {
        let mut resp = [0u8; RESP_MAX];
        let n = match self.request(OP_SCAN, &[], &mut resp, SCAN_TIMEOUT_MS) {
            Some(n) if n >= WIFI_HDR + STATS_LEN => n.min(RESP_MAX),
            _ => return (0, ScanOutcome::NoResponse, ScanStats::default()),
        };
        let stats = ScanStats {
            steps: word(&resp, WIFI_HDR),
            raw: word(&resp, WIFI_HDR + 4),
            beacons: word(&resp, WIFI_HDR + 8),
        };
        let mut count = 0;
        parse_scan(&resp[WIFI_HDR + STATS_LEN..n], |net| {
            if count < out.len() {
                out[count] = net;
                count += 1;
            }
        });
        (count, ScanOutcome::Scanned, stats)
    }
}
