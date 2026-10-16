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
use nonos_libc::{mk_idle_ms, Deadline};

use crate::constants::{PORTSC_CCS, PORTSC_PP};
use crate::controller::park::{IrqWait, SPIN_BUDGET};
use crate::regs::op::{portsc_neutral, portsc_read, portsc_write};

/// Root hub power-on to power-good: xHCI root hubs report 20 ms.
pub const POWER_GOOD_MS: u64 = 20;

/// Port power on, when the controller switches it (HCCPARAMS1.PPC) and it
/// is off, then wait for a connect up to the power-good time.
pub(super) fn power_on(op_base: u64, port: u8, irq: &mut IrqWait) -> u32 {
    let current = portsc_read(op_base, port);
    if (current & PORTSC_PP) == 0 {
        portsc_write(op_base, port, portsc_neutral(current) | PORTSC_PP);
    }
    let mut last = portsc_read(op_base, port);
    let deadline = Deadline::after_ms(POWER_GOOD_MS);
    let mut spins = SPIN_BUDGET;
    while (last & PORTSC_CCS) == 0 && !deadline.expired() {
        spins = spins.saturating_add(1);
        irq.park(spins);
        last = portsc_read(op_base, port);
    }
    last
}

/// Power every root port the controller left off, and give them the
/// power-good time, so a device attached at boot shows its connect before
/// any class driver looks. A controller without port power switching
/// (HCCPARAMS1.PPC clear) keeps PP at one and this writes nothing.
pub fn power_all_ports(op_base: u64, max_ports: u8) {
    let mut wrote = false;
    for port in 1..=max_ports {
        let current = portsc_read(op_base, port);
        if current & PORTSC_PP == 0 {
            portsc_write(op_base, port, portsc_neutral(current) | PORTSC_PP);
            wrote = true;
        }
    }
    if wrote {
        let _ = mk_idle_ms(POWER_GOOD_MS);
    }
}
