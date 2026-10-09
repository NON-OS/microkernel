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

//! The default receive queue of an AX210-family part (Linux v6.12 pcie/rx.c
//! for `IWL_DEVICE_FAMILY_AX210`).
//!
//! The host posts buffers on the free ring (`iwl_rx_transfer_desc`: buffer id,
//! address); the device fills one, writes a completion descriptor
//! (`iwl_rx_completion_desc`: buffer id, fragmented flag) on the used ring and
//! advances the closed index in the status word. The host walks the used ring
//! up to that index, takes each buffer and posts it again. The free ring's
//! write index goes to the device in multiples of eight, and only after the
//! ALIVE interrupt: the firmware configures the receive engine itself and
//! Linux restocks then (`iwl_pcie_rxmq_restock` on ALIVE).
//!
//! Everything the device writes is checked: a buffer id that is zero, out of
//! range or not currently posted is refused (Linux forces an NMI there), and
//! the closed index is masked to the ring.

use super::plan::{RB_SIZE, RB_STTS, RX_BUFS, RX_FREE, RX_FREE_DESC, RX_RING, RX_USED, RX_USED_DESC};
use super::region::{get16, put16, put64, zero, Region};
use super::regs::RFH_Q0_FRBDCB_WIDX_TRG;
use crate::regs::Mmio;

/// `IWL_RX_CD_FLAGS_FRAGMENTED`: this buffer is part of a frame larger than
/// one buffer. Such frames are dropped, as Linux does.
const CD_FRAGMENTED: u8 = 1 << 0;

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
    read: usize,
    write: usize,
    write_actual: usize,
    posted: [bool; RX_BUFS],
    next_is_fragment: bool,
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
    fn post<R: Region + ?Sized>(&mut self, ctrl: &R, rbs: &R, vid: u16) -> bool {
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
