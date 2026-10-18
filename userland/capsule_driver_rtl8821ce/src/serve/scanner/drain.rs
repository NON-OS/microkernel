// NONOS Operating System (AGPL-3.0-or-later)

//! Taking what the ring holds on the current channel into the picture.

use nonos_wifi_core::dot11::parse::parse_beacon;

use crate::fw::dma::DmaMem;
use crate::link::{RtlLink, RxBuffers};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Mmio;
use crate::scan;

use super::channels::SCAN_CHANNELS;
use super::state::Scanner;

/// Frames to drain from the ring on one pass before yielding back to the serve
/// loop, so a busy channel never starves request handling.
const DRAIN_PER_STEP: usize = 16;

impl Scanner {
    // Drain whatever the hardware has DMA-ed onto the ring on the current channel.
    // Run every serve-loop pass, request or not, so heavy net_core polling can
    // never starve the scan of the ring. Tunes the synthesizer on the first pass.
    // Nothing here transmits: the ring is only read (`RtlLink::poll_raw`) and
    // the synthesizer only tuned.
    pub(in super::super) fn drain<M: Mmio, D: DmaMem, RB: RxBuffers>(
        &mut self,
        link: &mut RtlLink<M, D, RB>,
        regs: &M,
    ) {
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
}
