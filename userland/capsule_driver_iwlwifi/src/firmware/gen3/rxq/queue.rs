// NONOS Operating System (AGPL-3.0-or-later)

//! The queue's state, and its rings cleared, stocked and kicked.

use super::super::plan::{RB_STTS, RX_BUFS, RX_FREE, RX_FREE_DESC, RX_RING, RX_USED, RX_USED_DESC};
use super::super::region::{zero, Region};
use super::super::regs::RFH_Q0_FRBDCB_WIDX_TRG;
use crate::regs::Mmio;

/// `IWL_RX_CD_FLAGS_FRAGMENTED`: this buffer is part of a frame larger than
/// one buffer. Such frames are dropped, as Linux does.
pub(super) const CD_FRAGMENTED: u8 = 1 << 0;

/// What one used-ring entry held.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Taken {
    /// A whole packet, copied out.
    Buffer,
    /// A piece of a multi-buffer frame, dropped.
    Fragment,
    /// The device named a buffer that is not posted; skipped.
    BadId,
}

pub struct RxQueue {
    pub(super) read: usize,
    pub(super) write: usize,
    pub(super) write_actual: usize,
    pub(super) posted: [bool; RX_BUFS],
    pub(super) next_is_fragment: bool,
}

impl Default for RxQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl RxQueue {
    pub const fn new() -> Self {
        Self { read: 0, write: 0, write_actual: 0, posted: [false; RX_BUFS], next_is_fragment: false }
    }

    /// Clear the status word and both rings and post every buffer on the free
    /// ring. The device is not told until [`RxQueue::kick`].
    pub fn init<R: Region + ?Sized>(&mut self, ctrl: &R, rbs: &R) -> bool {
        *self = Self::new();
        if !zero(ctrl, RB_STTS, 2)
            || !zero(ctrl, RX_FREE, RX_RING * RX_FREE_DESC)
            || !zero(ctrl, RX_USED, RX_RING * RX_USED_DESC)
        {
            return false;
        }
        (1..=RX_BUFS as u16).all(|vid| self.post(ctrl, rbs, vid))
    }

    /// Tell the device how far the free ring is stocked (a multiple of eight).
    pub fn kick<M: Mmio>(&mut self, m: &M) {
        let actual = self.write & !7;
        if actual != self.write_actual {
            self.write_actual = actual;
            m.write32(RFH_Q0_FRBDCB_WIDX_TRG, actual as u32);
        }
    }
}
