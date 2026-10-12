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

//! The controller's own interrupt, bound through the broker, which senses
//! the line as its `_CRS` says (the record's trigger and polarity bits).
//! A controller whose line will not bind is read on the old cadence.

use crate::bank::Bank;
use nonos_libc::{mk_irq_ack, mk_irq_bind, mk_irq_poll};
use nonos_libc::{DeviceRecord, IrqBindOut, IrqPollOut};

const NO_LINE: u8 = 0xFF;

/// The grant for the record's line; 0 for none, or a refused bind.
pub fn bind(r: &DeviceRecord, epoch: u64) -> u64 {
    if r.irq_pin == 0 || r.irq_line == NO_LINE {
        return 0;
    }
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    let n = mk_irq_bind(r.device_id, epoch, u32::from(r.irq_line), 0, 0, &mut out);
    if n < 0 {
        0
    } else {
        out.grant_id
    }
}

/// Whether the bank should be read now: its interrupt moved since it was
/// last read, or it has no interrupt and is read every pass.
pub fn fired(b: &mut Bank) -> bool {
    if b.grant == 0 {
        return true;
    }
    let mut p = IrqPollOut { seq: 0, overflow: 0 };
    if mk_irq_poll(b.grant, &mut p) < 0 || p.seq == b.seq {
        return false;
    }
    b.seq = p.seq;
    true
}

/// Lets the line fire again once its pins have been read.
pub fn ack(b: &Bank) {
    if b.grant != 0 {
        let _ = mk_irq_ack(b.grant);
    }
}
