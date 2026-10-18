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

//! What can go wrong under a taken-over output while it runs (PLAN G 12):
//! the pipe's FIFO ran dry, the transcoder stopped, or the plane is no
//! longer scanning out the surface last flipped to. Each is read without
//! changing the picture; the underrun's sticky bit is cleared as it is
//! read, so each underrun is reported once.

use super::access::Mmio;
use super::gen::{Gen, Pte};
use super::pipe::{Pipe, Transcoder};
use super::plane_regs as r;
use super::regs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    Underrun,
    Stopped,
    Hung,
}

/// DE_PIPE_IIR of `p` from Broadwell on (SKL PRM Vol 2c, DE_PIPE_IIR:
/// 0x44408 stepping 0x10 per pipe); Haswell reports underruns elsewhere
/// and is not read here.
pub fn de_pipe_iir(g: &Gen, p: Pipe) -> Option<u32> {
    (g.ggtt == Pte::Wide).then_some(0x4_4408 + 0x10 * p.index())
}

/// Pipe FIFO Underrun, bit 31, write one to clear (bit unverified).
pub const UNDERRUN: u32 = 1 << 31;

/// The worst fault on pipe `p`, fed by `t`, which should be showing
/// `front`: a stopped transcoder first, then a hung flip, then an
/// underrun, which leaves the picture standing.
pub fn check(m: &mut dyn Mmio, g: &Gen, p: Pipe, t: Transcoder, front: u32) -> Option<Fault> {
    let underrun = de_pipe_iir(g, p).is_some_and(|iir| {
        let hit = m.read(iir) & UNDERRUN != 0;
        if hit {
            m.write(iir, UNDERRUN);
        }
        hit
    });
    if m.read(regs::trans_conf(t)) & (1 << 30) == 0 {
        return Some(Fault::Stopped);
    }
    if m.read(r::plane_surflive(p, 1)) & !0xFFF != front {
        return Some(Fault::Hung);
    }
    underrun.then_some(Fault::Underrun)
}
