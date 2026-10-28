// NONOS Operating System (AGPL-3.0-or-later)

//! Filled buffers taken off the used ring and posted again.

use super::super::plan::{RB_SIZE, RB_STTS, RX_BUFS, RX_RING};
use super::super::region::{get16, zero, Region};
use super::queue::{RxQueue, Taken, CD_FRAGMENTED};

impl RxQueue {
    /// Take the next filled buffer: copy it into `out` (at least
    /// [`RB_SIZE`] bytes) and post it again. `None` when nothing is pending.
    pub fn take<R: Region + ?Sized>(&mut self, ctrl: &R, rbs: &R, out: &mut [u8]) -> Option<Taken> {
        let closed = closed(ctrl)?;
        if closed == self.read {
            return None;
        }
        let at = self.read;
        self.read = (self.read + 1) & (RX_RING - 1);
        let (vid, flags) = self.used(ctrl, at)?;
        // A failed read must not pass as a clear flag: a zero there would take
        // one piece of a multi-buffer frame as a whole one (E-34).
        let Some(flags) = flags else { return Some(Taken::BadId) };
        let slot = (vid as usize).wrapping_sub(1);
        if !self.posted.get(slot).copied().unwrap_or(false) {
            return Some(Taken::BadId);
        }
        self.posted[slot] = false;
        let fragment = flags & CD_FRAGMENTED != 0 || self.next_is_fragment;
        self.next_is_fragment = flags & CD_FRAGMENTED != 0;
        let stride = self.csr.rb_stride();
        let copied = out.get_mut(..RB_SIZE).is_some_and(|o| rbs.read(slot * stride, o));
        // Nothing of the frame stays in the buffer the device gets back.
        if !zero(rbs, slot * stride, stride) || !self.post(ctrl, rbs, vid) || !copied {
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
        let addr = rbs.dev() + (slot * self.csr.rb_stride()) as u64;
        if !self.free(ctrl, self.write, vid, addr) {
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
