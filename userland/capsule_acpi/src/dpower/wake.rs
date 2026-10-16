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

//! Runtime wake for a driver's own device (`nonos_aml::device_power`
//! runtime wake): armed and disarmed by its holder through DEVICE_POWER,
//! its wake event enabled through the event layer, and each Notify(dev,
//! 2) counted for the holder to read (`wake_check.rs`).

use crate::boot::{evaluator, path, Acpi};
use crate::events_io::PortIo;
use crate::server::protocol::{EINVAL, EIO, ENOENT};
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::device_power::{DState, GpeSwitch, RuntimeWake, WakeError};
use nonos_aml::events::Events;
use nonos_aml::namespace::Id;

#[derive(Default)]
pub struct Wakes {
    pub rw: RuntimeWake,
    /// Each armed device, its holder's pid and the device id it named.
    pub owners: Vec<(Id, u32, u64)>,
    pub check_at: u64,
}

struct Gpes<'a>(&'a mut Option<Events>);

impl GpeSwitch for Gpes<'_> {
    fn switch(&mut self, gpe: u16, on: bool) -> bool {
        self.0.as_mut().is_some_and(|e| e.arm_wake(&mut PortIo, gpe, on).is_ok())
    }
}

pub fn wake(acpi: &mut Acpi, dev: Id, on: Option<DState>, owner: (u32, u64)) -> Result<(), WakeError> {
    let Acpi { aml, host, cooling, events, hidden, .. } = acpi;
    let (ev, pool, w) = (&mut evaluator(host), &mut cooling.pool, &mut hidden.wake);
    w.owners.retain(|o| o.0 != dev);
    let r = match on {
        Some(to) => w.rw.arm((pool, aml, ev), &mut Gpes(events), dev, to),
        None => {
            w.rw.disarm((pool, aml, ev), &mut Gpes(events), dev);
            Ok(())
        }
    };
    if on.is_some() && r.is_ok() {
        w.owners.push((dev, owner.0, owner.1));
    }
    let l = Line::new(b"wake ").str(path(aml, dev).as_bytes());
    let l = l.str(if on.is_some() { b" armed" } else { b" disarmed" });
    l.str(if r.is_ok() { b"" } else { b": refused" }).str(b" for pid ").dec(owner.0.into()).end();
    r
}

/// DEVICE_POWER's status for a wake verb.
pub fn code(r: Result<(), WakeError>) -> i32 {
    match r {
        Ok(()) => 0,
        Err(WakeError::NoEvent) => ENOENT,
        Err(WakeError::TooDeep(_)) => EINVAL,
        Err(_) => EIO,
    }
}
