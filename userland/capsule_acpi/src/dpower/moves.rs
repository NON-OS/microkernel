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

//! A device's state as the capsule knows it, and a move made for its
//! driver, through the power-resource pool the fans share.

use crate::boot::{evaluator, path, Acpi};
use crate::server::protocol::{EINVAL, EIO};
use crate::sys::log::Line;
use nonos_aml::device_power::{power_managed, psc, DState, MoveError};
use nonos_aml::namespace::Id;

/// What this capsule last set; else _PSC; else D0 for a device with no
/// power control, which is always on; else unknown.
pub fn current(acpi: &mut Acpi, dev: Id) -> Option<DState> {
    if let Some(s) = acpi.dpower.state(dev) {
        return Some(s);
    }
    let Acpi { aml, host, .. } = acpi;
    psc(aml, &mut evaluator(host), dev).or((!power_managed(aml, dev)).then_some(DState::D0))
}

pub fn move_to(acpi: &mut Acpi, dev: Id, to: DState, pid: u32) -> i32 {
    // A device firmware left in an unknown state is taken to be off, so
    // its D0 runs every step.
    let from = current(acpi, dev).unwrap_or(DState::D3Cold);
    let Acpi { aml, host, dpower, cooling, .. } = acpi;
    let r = dpower.set(&mut cooling.pool, aml, &mut evaluator(host), dev, from, to);
    let l = Line::new(b"power ").str(path(aml, dev).as_bytes()).str(b" ");
    let l = l.dec(from as u64).str(b" -> ").dec(to as u64).str(b" for pid ").dec(u64::from(pid));
    match r {
        Ok(()) => {
            l.end();
            0
        }
        Err(MoveError::Fault(f)) => {
            l.str(b": failed at ").hex(u64::from(f.at)).end();
            EIO
        }
        Err(_) => {
            l.str(b": refused").end();
            EINVAL
        }
    }
}
