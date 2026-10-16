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

//! A move made for a device's driver, through the power-resource pool the
//! fans share.

use super::current::current;
use crate::boot::{evaluator, path, Acpi};
use crate::server::protocol::{EINVAL, EIO, ELOSES};
use crate::sys::log::Line;
use nonos_aml::device_power::{DState, MoveError};
use nonos_aml::namespace::Id;

pub fn move_to(acpi: &mut Acpi, dev: Id, to: DState, (pid, id): (u32, u64)) -> i32 {
    // A device firmware left in an unknown state is taken to be off, so
    // its D0 runs every step.
    let from = current(acpi, dev).unwrap_or(DState::D3Cold);
    if !super::keep::keeps(acpi, dev, from, to) {
        let l = Line::new(b"power ").str(path(&acpi.aml, dev).as_bytes()).str(b": ");
        l.str(b"refused, the function's configuration would be lost").end();
        return ELOSES;
    }
    let Acpi { aml, host, dpower, cooling, .. } = acpi;
    let r = dpower.set(&mut cooling.pool, aml, &mut evaluator(host), dev, from, to);
    let waited = r.is_ok().then(|| super::ready::settle_after(acpi, dev, from, to)).flatten();
    let l = Line::new(b"power ").str(path(&acpi.aml, dev).as_bytes()).str(b" ");
    let l = l.dec(from as u64).str(b" -> ").dec(to as u64).str(b" for pid ").dec(u64::from(pid));
    let l = match waited {
        Some(us) => l.str(b", ready after ").dec(u64::from(us)).str(b"us"),
        None => l,
    };
    match r {
        Ok(()) => match super::keep::restore_after(acpi, dev, id, from, to) {
            Ok(()) => {
                l.end();
                0
            }
            Err(e) => {
                l.str(b": BARs not put back, errno ").dec(e.unsigned_abs()).end();
                // The kernel's EIO: a BAR would not stick and the claim went.
                if e == i64::from(EIO) {
                    ELOSES
                } else {
                    EIO
                }
            }
        },
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
