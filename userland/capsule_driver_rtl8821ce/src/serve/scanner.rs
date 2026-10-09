// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The background scanner: a running, deduplicated picture of the networks in
//! range that the serve loop fills a little at a time between requests, so a scan
//! request is answered instantly from the cache instead of blocking the caller for
//! a full channel sweep. It only advances while the radio is up and not
//! associated, so hopping channels never disturbs a live connection.

use nonos_wifi_core::dot11::parse::parse_beacon;

use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::phy::channel::{set_rf, Bw};
use crate::regs::{Mmio, Regs};
use crate::scan;

use super::{SCAN_CHANNELS, SCAN_FRAME_MAX};

/// How long to dwell on one channel before hopping, in milliseconds of uptime:
/// enough to catch beacons (about ten a second) without a long sweep. It is
/// counted on the clock, not in idle passes: with every core running, net_core
/// asks the driver for frames every 20 ms, a pass is never idle, and a hop that
/// waited for one left the scan on channel 1 for good.
const DWELL_MS: i64 = 300;
/// Frames to drain from the ring on one pass before yielding back to the serve
/// loop, so a busy channel never starves request handling.
const DRAIN_PER_STEP: usize = 16;
/// Sweeps whose totals go on the log, so a scan that hears nothing on a real
/// laptop says so in `log rtl8821ce` without filling the ring.
const SWEEPS_SAID: u32 = 3;

pub(super) struct Scanner {
    results: scan::ScanResults,
    frame: [u8; SCAN_FRAME_MAX],
    ch_idx: usize,
    /// Uptime at which the scan moves to the next channel.
    hop_at_ms: i64,
    tuned: bool,
    // Counters, so a serial-less boot can see where the receive path stops: how
    // many passes ran, how many raw frames the ring delivered, and how many of
    // those parsed as beacons.
    pub(super) steps: u32,
    pub(super) raw: u32,
    pub(super) beacons: u32,
    sweeps: u32,
}

impl Scanner {
    pub(super) fn new() -> Self {
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

    // Drain whatever the hardware has DMA-ed onto the ring on the current channel.
    // Run every serve-loop pass, request or not, so heavy net_core polling can
    // never starve the scan of the ring. Tunes the synthesizer on the first pass.
    pub(super) fn drain(&mut self, link: &mut RtlLink<Regs, Grant, Grant>, regs: &Regs) {
        if !self.tuned {
            set_rf(regs, SCAN_CHANNELS[self.ch_idx], Bw::W20);
            self.tuned = true;
        }
        self.steps = self.steps.saturating_add(1);
        for _ in 0..DRAIN_PER_STEP {
            match link.poll_raw(&mut self.frame) {
                Some(n) => {
                    self.raw = self.raw.saturating_add(1);
                    let frame = &self.frame[..n];
                    if let Some(b) = parse_beacon(frame) {
                        self.beacons = self.beacons.saturating_add(1);
                        self.results.add_bss(b.bssid, b.ssid, scan::beacon_flags(frame, b.capability));
                    }
                }
                None => break,
            }
        }
    }

    // Advance the channel hop once the current channel has had its dwell on the
    // clock. Called on every pass while unassociated, busy or idle.
    pub(super) fn advance(&mut self, regs: &Regs) {
        let now = nonos_libc::mk_uptime_ms();
        if self.hop_at_ms == 0 {
            self.hop_at_ms = now + DWELL_MS;
            return;
        }
        if now < self.hop_at_ms {
            return;
        }
        self.ch_idx = (self.ch_idx + 1) % SCAN_CHANNELS.len();
        if self.ch_idx == 0 {
            self.results.end_sweep();
            self.say_sweep(regs);
        }
        set_rf(regs, SCAN_CHANNELS[self.ch_idx], Bw::W20);
        self.hop_at_ms = now + DWELL_MS;
    }

    // The running totals after each of the first sweeps of the 13 channels,
    // and the receive ring's index register as the card holds it: its write
    // index in bits 16..27 and ours in 0..11. A write index that stays put
    // while frames= does not move is a card that stopped receiving; one that
    // moves on is a ring this driver is not reading.
    fn say_sweep(&mut self, regs: &Regs) {
        self.sweeps = self.sweeps.saturating_add(1);
        if self.sweeps > SWEEPS_SAID {
            return;
        }
        crate::status::line(b"[rtl8821ce] scan sweep ");
        crate::status::number(self.sweeps);
        crate::status::line(b": frames=");
        crate::status::number(self.raw);
        crate::status::line(b" beacons=");
        crate::status::number(self.beacons);
        let idx = regs.read32(crate::rx::regs::REG_RXBD_IDX_MPDUQ);
        crate::status::line(b" rxbd_idx=0x");
        crate::status::hex16((idx >> 16) as u16);
        crate::status::hex16(idx as u16);
        crate::status::line(b"\n");
    }

    pub(super) fn cache(&self) -> &scan::ScanResults {
        &self.results
    }
}
