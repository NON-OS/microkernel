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

//! What a driver's armed device hears: each Notify(dev, 2) counted. Every
//! two seconds each armed device's holder is asked again (`wake_sweep`),
//! and one that no longer holds it, by release, exit or departure, has
//! its wake disarmed.

use super::target::target;
use super::wake::wake;
use super::wake::Gpes;
use super::wake_sweep::{departed as gone, stale};
use crate::boot::{evaluator, path, Acpi};
use crate::server::protocol::{EINVAL, EIO, ENOENT};
use crate::sys::log::Line;
use crate::sys::clock::now_ms;
use crate::sys::function_of::Asker;
use nonos_aml::device_power::WakeError;
use nonos_aml::namespace::Id;

const CHECK_MS: u64 = 2_000;

/// A Notify(dev, 2): counted when its driver armed it.
pub fn woken(acpi: &mut Acpi, id: Id, value: u8) -> bool {
    value == 2 && acpi.hidden.wake.rw.woken(id)
}

pub fn tick(acpi: &mut Acpi) {
    let now = now_ms();
    if acpi.hidden.wake.owners.is_empty() || now < acpi.hidden.wake.check_at {
        return;
    }
    acpi.hidden.wake.check_at = now + CHECK_MS;
    let owners = acpi.hidden.wake.owners.clone();
    for (dev, pid, id) in stale(&owners, &mut |pid, id| target(acpi, Asker::Holder(pid), id).ok()) {
        let _ = wake(acpi, dev, None, (pid, id));
    }
}

/// Devices that just departed: each one's wake dropped at once, its event
/// off with its last user and its resources given back, and no AML run
/// on the device itself (`RuntimeWake::forget`).
pub fn departed(acpi: &mut Acpi, ids: &[u64]) {
    for (dev, pid, _) in gone(&acpi.hidden.wake.owners, ids) {
        let Acpi { aml, host, cooling, events, hidden, .. } = acpi;
        hidden.wake.owners.retain(|o| o.0 != dev);
        let ev = &mut evaluator(host);
        hidden.wake.rw.forget((&mut cooling.pool, aml, ev), &mut Gpes(events), dev);
        let l = Line::new(b"wake ").str(path(aml, dev).as_bytes());
        l.str(b" dropped: the device left, pid ").dec(pid.into()).end();
    }
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
