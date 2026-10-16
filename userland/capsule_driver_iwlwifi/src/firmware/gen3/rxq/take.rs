// NONOS Operating System (AGPL-3.0-or-later)

//! Filled buffers taken off the used ring and posted again.

use super::super::plan::{
    RB_SIZE, RB_STTS, RX_BUFS, RX_FREE, RX_FREE_DESC, RX_RING, RX_USED, RX_USED_DESC,
};
use super::super::region::{get16, put16, put64, Region};
use super::queue::{RxQueue, Taken, CD_FRAGMENTED};

impl RxQueue {
    /// Take the next filled buffer: copy it into `out` (at least
    /// [`RB_SIZE`] bytes) and post it again. `None` when nothing is pending.
    pub fn take<R: Region + ?Sized>(&mut self, ctrl: &R, rbs: &R, out: &mut [u8]) -> Option<Taken> {
        let closed = closed(ctrl)?;
        if closed == self.read {
            return None;
        }
        let cd = RX_USED + self.read * RX_USED_DESC;
        self.read = (self.read + 1) & (RX_RING - 1);
        let vid = get16(ctrl, cd + 4)?;
        let mut flags = [0u8; 1];
        ctrl.read(cd + 6, &mut flags);
        let slot = (vid as usize).wrapping_sub(1);
        if !self.posted.get(slot).copied().unwrap_or(false) {
            return Some(Taken::BadId);
        }
        self.posted[slot] = false;
        let fragment = flags[0] & CD_FRAGMENTED != 0 || self.next_is_fragment;
        self.next_is_fragment = flags[0] & CD_FRAGMENTED != 0;
        let copied = out.get_mut(..RB_SIZE).is_some_and(|o| rbs.read(slot * RB_SIZE, o));
        if !self.post(ctrl, rbs, vid) || !copied {
            return Some(Taken::BadId);
        }
        Some(if fragment { Taken::Fragment } else { Taken::Buffer })
    }

    // Put buffer `vid` (1-based) on the free ring.
    pub(super) fn post<R: Region + ?Sized>(&mut self, ctrl: &R, rbs: &R, vid: u16) -> bool {
        let slot = (vid as usize).wrapping_sub(1);
        if slot >= RX_BUFS {
            return false;
        }
        let at = RX_FREE + self.write * RX_FREE_DESC;
        let addr = rbs.dev() + (slot * RB_SIZE) as u64;
        if !put16(ctrl, at, vid) || !put64(ctrl, at + 8, addr) {
            return false;
        }
        self.posted[slot] = true;
        self.write = (self.write + 1) & (RX_RING - 1);
        true
    }
}
// The closed index the device last wrote, masked to the ring.
fn closed<R: Region + ?Sized>(ctrl: &R) -> Option<usize> {
    get16(ctrl, RB_STTS).map(|v| (v & 0x0FFF) as usize & (RX_RING - 1))
}
