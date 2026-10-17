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

//! Stopping an SD/eMMC host before its frames are freed (F-128), from the
//! SD Host Controller Simplified Specification as recalled, unverified:
//! the 8-bit Software Reset register at 0x2F, whose bit 0, Software Reset
//! For All, resets the whole controller, its DMA engine with it, and reads
//! back 1 until the reset is done. The read-back that shows it done is
//! taken as the barrier: a read of the device does not pass its own
//! earlier writes on the fabrics these hosts sit on, recalled, unverified.
//! A host that does not finish, or cannot be reached, keeps its frames:
//! they are never freed under a master that may still write to them. A
//! register that reads all ones, as a gone device answers, never reads as
//! done. Pure: `kernel_proofs` mounts it.

pub(crate) const SOFTWARE_RESET: u64 = 0x2F;
pub(crate) const RESET_ALL: u8 = 1 << 0;
/// Our choice, from no source: far past the time a reset takes.
pub(crate) const BOUND_MS: u64 = 100;
/// Our choice, from no source: a clock that does not move still ends.
pub(crate) const MAX_READS: u32 = 1_000_000;

/// The host's window, as the broker reaches it.
pub(crate) trait Window {
    /// None when the window cannot be reached (fenced, gone).
    fn read8(&mut self, off: u64) -> Option<u8>;
    fn write8(&mut self, off: u64, v: u8) -> bool;
    fn now_ms(&mut self) -> u64;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    /// The reset read back done: the frames may be freed.
    Stopped,
    /// Still resetting at the bound: the frames are kept.
    StillRunning,
    /// The window could not be written or read: the frames are kept.
    Unreachable,
}

pub(crate) fn reset_all(w: &mut impl Window) -> Outcome {
    if !w.write8(SOFTWARE_RESET, RESET_ALL) {
        return Outcome::Unreachable;
    }
    let start = w.now_ms();
    for _ in 0..MAX_READS {
        match w.read8(SOFTWARE_RESET) {
            None => return Outcome::Unreachable,
            Some(v) if v & RESET_ALL == 0 => return Outcome::Stopped,
            Some(_) => {}
        }
        if w.now_ms().saturating_sub(start) >= BOUND_MS {
            break;
        }
    }
    Outcome::StillRunning
}

/// Whether the frames may be freed after `o`.
pub(crate) const fn frees(o: Outcome) -> bool {
    matches!(o, Outcome::Stopped)
}
