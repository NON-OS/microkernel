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

//! A stalled EC tried again once its back-off has passed (`Health::retry`):
//! one line; its GPE armed again if the stall had it quieted; and the
//! batteries and adapters read at once, since their methods read the EC.

use crate::boot::Acpi;
use crate::events_io::PortIo;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;

pub fn retry(acpi: &mut Acpi) {
    if acpi.ec.is_none() || !acpi.host.ec_health.retry(now_ms()) {
        return;
    }
    let trial = acpi.host.ec_health.trials();
    Line::new(b"EC on trial again after a stall: trial ").dec(u64::from(trial)).end();
    let gpe = acpi.ec.as_ref().and_then(|e| e.gpe);
    if let (Some(ev), Some(g)) = (acpi.events.as_mut(), gpe) {
        if ev.native.contains(&g)
            && !ev.armed.contains(&g)
            && ev.arm_native(&mut PortIo, g, false).is_err()
        {
            Line::new(b"EC GPE ").hex(u64::from(g)).str(b" not armed again").end();
        }
    }
    acpi.power.read_ms = 0;
}
