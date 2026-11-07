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

//! The function's interrupt for the vertical blank: bound once after the
//! claim (MSI, else the routed INTx line), slept on by the flips through
//! the engine's `Park`. No grant leaves every pipe polled.

use nonos_intel_display::engine::park::{Park, Woke};
use nonos_libc::MK_IRQ_WAIT_TIMED_OUT;
use nonos_libc::{mk_irq_ack, mk_irq_bind, mk_irq_wait, IrqBindOut, MK_IRQ_BIND_MSI};

use crate::discover::Found;

const NO_LINE: u8 = 0xFF;

/// The grant, 0 when none was bound.
pub fn bind(dev: &Found, epoch: u64) -> u64 {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    if mk_irq_bind(dev.device_id, epoch, 0, MK_IRQ_BIND_MSI, 1, &mut out) >= 0 {
        return out.grant_id;
    }
    let routed = dev.irq_pin != 0 && dev.irq_line != NO_LINE;
    if routed && mk_irq_bind(dev.device_id, epoch, dev.irq_line as u32, 0, 0, &mut out) >= 0 {
        return out.grant_id;
    }
    0
}

pub struct Vblank {
    grant: u64,
    seq: u64,
}

impl Vblank {
    pub const fn new(grant: u64) -> Self {
        Vblank { grant, seq: 0 }
    }
}

impl Park for Vblank {
    fn bound(&self) -> bool {
        self.grant != 0
    }

    fn wait(&mut self, ms: u64) -> Woke {
        // Grant 0 would wait on every grant the capsule holds.
        if self.grant == 0 {
            return Woke::Refused;
        }
        let mut seq = self.seq;
        let rc = mk_irq_wait(self.grant, self.seq, ms, &mut seq);
        let moved = seq != self.seq;
        self.seq = seq;
        match rc {
            r if r < 0 => {
                self.grant = 0;
                Woke::Refused
            }
            _ if moved => Woke::Irq,
            MK_IRQ_WAIT_TIMED_OUT => Woke::TimedOut,
            _ => Woke::Early,
        }
    }

    fn ack(&mut self) {
        if self.grant != 0 {
            let _ = mk_irq_ack(self.grant);
        }
    }
}
