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

//! A periodic QH stopped. It loses its micro-frame mask and a frame is let
//! pass, after which the controller has no transaction of it in flight and
//! it is emptied. A frame that never passes is judged as an unanswered
//! doorbell is (advance.rs): a halted controller runs nothing; past that it
//! is stopped as a dead one is, and one that will not stop keeps the QH as
//! it was, since the copy it may hold would be written back over a new one.

use super::advance::{stop, Advance, Dead};
use super::qh::empty;
use crate::env::{until, Clock, Mem, Regs};
use crate::facts::regs::{FRINDEX, STS_HALTED, USBSTS};

/// A frame is eight micro-frames, FRINDEX's unit; one passes within this
/// bound on a running controller (this engine's own).
const FRAME_MS: u64 = 3;

/// The QH at `at` parked: emptied with DATA0 next once a frame passed, or
/// once the controller is halted. Err when the frame never came; the QH is
/// emptied only if the controller was then seen to halt.
pub fn park<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    op: u32,
    at: usize,
) -> Result<(), Dead> {
    m.wr(at + 8, m.rd(at + 8) & !0xFF);
    let start = r.r32(op + FRINDEX);
    let passed = until(c, FRAME_MS, || r.r32(op + FRINDEX).wrapping_sub(start) & 0x3FFF >= 8);
    let sts = r.r32(op + USBSTS);
    let verdict = match passed || (sts != !0 && sts & STS_HALTED != 0) {
        true => Advance::Seen,
        false if stop(r, c, op) => Advance::Stopped,
        false => Advance::Wedged,
    };
    if verdict != Advance::Wedged {
        empty(m, at, false);
    }
    match verdict {
        Advance::Seen => Ok(()),
        _ => Err(Dead),
    }
}
