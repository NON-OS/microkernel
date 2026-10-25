// NONOS Operating System (AGPL-3.0-or-later)

//! Moving to the next channel on the clock, and the sweep's log line.

use crate::phy::channel::{set_rf, Bw};
use crate::regs::{Mmio, Regs};

use super::channels::SCAN_CHANNELS;
use super::state::Scanner;

/// How long to dwell on one channel before hopping, in milliseconds of uptime:
/// enough to catch beacons (about ten a second) without a long sweep. It is
/// counted on the clock, not in idle passes: with every core running, net_core
/// asks the driver for frames every 20 ms, a pass is never idle, and a hop that
/// waited for one left the scan on channel 1 for good.
const DWELL_MS: i64 = 300;
/// Sweeps whose totals go on the log, so a scan that hears nothing on a real
/// laptop says so in `log rtl8821ce` without filling the ring.
const SWEEPS_SAID: u32 = 3;

impl Scanner {
    // Advance the channel hop once the current channel has had its dwell on the
    // clock. Called on every pass while unassociated, busy or idle.
    pub(in super::super) fn advance(&mut self, regs: &Regs) {
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
}
