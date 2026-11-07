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

//! The display engine's vertical blank interrupt, per pipe. Each pipe has
//! its DE_PIPE_IMR, IIR and IER from Broadwell on (BDW PRM Vol 2c,
//! DE_PIPE_*: 0x44404, 0x44408, 0x4440C, stepping 0x10 per pipe), VBLANK
//! at bit 0. Above them one master switch lets the event out as the
//! function's interrupt: Gen8 to Gen10 MASTER_INT_CTL at 0x44200 bit 31;
//! Gen11 on two, GFX_MSTR_INTR at 0x190010 bit 31 and DISPLAY_INT_CTL at
//! 0x44200 bit 31 (ICL PRM Vol 2c).

use super::access::{rmw, Mmio};
use super::gen::Gen;
use super::pipe::Pipe;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Master {
    Gen8,
    Gen11,
}

pub const VBLANK: u32 = 1;
pub const ENABLE: u32 = 1 << 31;
pub const MASTER_INT_CTL: u32 = 0x4_4200;
pub const GFX_MSTR_INTR: u32 = 0x19_0010;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IrqRegs {
    pub imr: u32,
    pub iir: u32,
    pub ier: u32,
    pub master: Master,
}

/// The interrupt registers of pipe `p`, or None where the pipes stay
/// polled: Haswell, and a pipe the generation does not have.
pub fn irq_regs(g: &Gen, p: Pipe) -> Option<IrqRegs> {
    let master = g.irq?;
    let base = 0x4_4400 + 0x10 * p.index();
    (p.index() < g.pipes).then_some(IrqRegs {
        imr: base + 4,
        iir: base + 8,
        ier: base + 0xC,
        master,
    })
}

/// The master switches, in the order they are turned on; off is the
/// reverse.
fn switches(m: Master) -> &'static [u32] {
    match m {
        Master::Gen8 => &[MASTER_INT_CTL],
        Master::Gen11 => &[MASTER_INT_CTL, GFX_MSTR_INTR],
    }
}

fn master(m: &mut dyn Mmio, which: Master, on: bool) {
    let all = switches(which);
    if on {
        all.iter().for_each(|&r| m.write(r, ENABLE));
    } else {
        all.iter().rev().for_each(|&r| m.write(r, 0));
    }
}

/// Let pipe `r`'s next vertical blank out as an interrupt: a stale event
/// is cleared first, so the one waited for is new.
pub fn arm(m: &mut dyn Mmio, r: &IrqRegs) {
    m.write(r.iir, VBLANK);
    rmw(m, r.imr, VBLANK, 0);
    rmw(m, r.ier, VBLANK, VBLANK);
    master(m, r.master, true);
}

/// Clear the event that woke the waiter. The master is off while the
/// IIR is cleared and on again after, so an event still standing raises
/// a new message rather than none (MSI is edge signalled).
pub fn ack(m: &mut dyn Mmio, r: &IrqRegs) {
    master(m, r.master, false);
    m.write(r.iir, VBLANK);
    master(m, r.master, true);
}

/// Stop the pipe raising vertical blanks, and close the master: nothing
/// interrupts an idle machine sixty times a second.
pub fn disarm(m: &mut dyn Mmio, r: &IrqRegs) {
    master(m, r.master, false);
    rmw(m, r.ier, VBLANK, 0);
    rmw(m, r.imr, VBLANK, VBLANK);
    m.write(r.iir, VBLANK);
}
