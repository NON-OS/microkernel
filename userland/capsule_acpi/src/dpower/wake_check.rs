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
//! two seconds each armed device's holder is asked again, and one that no
//! longer holds it has its wake disarmed, so nothing stays armed for a
//! driver that is gone.

use super::target::target;
use super::wake::wake;
use crate::boot::Acpi;
use crate::sys::clock::now_ms;
use crate::sys::function_of::Asker;
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
    for (dev, pid, id) in acpi.hidden.wake.owners.clone() {
        if target(acpi, Asker::Holder(pid), id) != Ok(dev) {
            let _ = wake(acpi, dev, None, (pid, id));
        }
    }
}
