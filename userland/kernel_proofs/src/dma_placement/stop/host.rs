// NONOS Operating System (AGPL-3.0-or-later)
//! A modelled SD host's window: its reset finishes after a number of reads
//! (or never), its clock moves a set step at each look, and it may answer
//! all ones, refuse the write, or go away after some reads.

use super::sdhci_stop::Window;

#[derive(Default)]
pub struct Host {
    /// Reads that still see the reset running; None, it never ends.
    pub busy_reads: Option<u32>,
    /// Ms the clock moves at each look; 0, it never moves.
    pub step: u64,
    pub now: u64,
    pub all_ones: bool,
    pub refuse_write: bool,
    /// Reads answered before the window goes away.
    pub gone_after: Option<u32>,
    pub writes: Vec<(u64, u8)>,
    pub reads: u32,
    /// The last value a read gave.
    pub last: Option<u8>,
}

impl Window for Host {
    fn read8(&mut self, off: u64) -> Option<u8> {
        assert_eq!(off, 0x2F, "only the reset register is read");
        assert_eq!(self.writes.len(), 1, "read before the reset was asked");
        if self.gone_after.is_some_and(|g| self.reads >= g) {
            self.last = None;
            return None;
        }
        let busy = self.busy_reads.is_none_or(|n| self.reads < n);
        self.reads += 1;
        let v = if self.all_ones { 0xFF } else { u8::from(busy) };
        self.last = Some(v);
        Some(v)
    }

    fn write8(&mut self, off: u64, v: u8) -> bool {
        self.writes.push((off, v));
        !self.refuse_write
    }

    fn now_ms(&mut self) -> u64 {
        self.now += self.step;
        self.now
    }
}
