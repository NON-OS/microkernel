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

//! A flip that sleeps until the vertical blank which latches it, instead
//! of polling PLANE_SURFLIVE for up to a frame. The poll stays behind it:
//! a pipe whose interrupt cannot be had, or did not come while its flip
//! landed, is polled from then on, so a machine where the interrupt never
//! fires pays one park once and not a frame on every flip.

use super::access::{Expiry, Mmio};
use super::error::Error;
use super::flip::{flip, wait_live};
use super::irq_regs::{ack, arm, disarm, IrqRegs};
use super::park::{Blank, Park, Woke};
use super::pipe::Pipe;
use super::plane::SURF_ALIGN;
use super::plane_regs as r;

/// One park's longest sleep: a frame at 24 Hz, the slowest refresh a
/// mode here runs, so any lit pipe blanks within it.
pub const FRAME_MS: u64 = 42;

/// Reads of PLANE_SURFLIVE after a wake before parking again: the
/// interrupt and the latch come from the same blank, and the read only
/// has to outrun the message.
pub const SETTLE_READS: u32 = 64;

fn live(m: &mut dyn Mmio, p: Pipe, surf: u32) -> bool {
    m.read(r::plane_surflive(p, 1)) & !0xFFF == surf
}

fn settled(m: &mut dyn Mmio, p: Pipe, surf: u32) -> bool {
    (0..SETTLE_READS).any(|_| live(m, p, surf))
}

/// Flip pipe `p` to `surf`, sleeping on its vertical blank when `blank`
/// says the interrupt works there; `blank` turns to `Poll` for good when
/// it does not.
pub fn flip_parked(
    m: &mut dyn Mmio,
    p: Pipe,
    surf: u32,
    until: &dyn Expiry,
    irq: (Option<IrqRegs>, &mut dyn Park, &mut Blank),
) -> Result<(), Error> {
    let (regs, park, blank) = irq;
    let Some(regs) = regs.filter(|_| *blank == Blank::Irq && park.bound()) else {
        return flip(m, p, surf, until);
    };
    if !surf.is_multiple_of(SURF_ALIGN) {
        return Err(Error::Argument);
    }
    arm(m, &regs);
    m.write(r::plane_surf(p, 1), surf);
    let landed = sleep_until_live(m, p, surf, until, (&regs, park, blank));
    disarm(m, &regs);
    match landed {
        Some(done) => done,
        None => wait_live(m, p, surf, until),
    }
}

/// Parks until the flip is live; None hands the rest of the wait to the
/// poll, once the interrupt is known not to serve this pipe.
fn sleep_until_live(
    m: &mut dyn Mmio,
    p: Pipe,
    surf: u32,
    until: &dyn Expiry,
    irq: (&IrqRegs, &mut dyn Park, &mut Blank),
) -> Option<Result<(), Error>> {
    let (regs, park, blank) = irq;
    loop {
        if live(m, p, surf) {
            return Some(Ok(()));
        }
        if until.expired() {
            return Some(Err(Error::Timeout));
        }
        match park.wait(FRAME_MS) {
            Woke::Irq => {
                ack(m, regs);
                park.ack();
                if settled(m, p, surf) {
                    return Some(Ok(()));
                }
            }
            Woke::Early => {}
            // A blank passed without a message if the flip landed.
            Woke::TimedOut if live(m, p, surf) => {
                *blank = Blank::Poll;
                return Some(Ok(()));
            }
            Woke::TimedOut => {}
            Woke::Refused => {
                *blank = Blank::Poll;
                return None;
            }
        }
    }
}
