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

//! The claim's scope check (K6f): the claimant's process, by every service
//! name the kernel registered for it, against scope.rs.

use crate::hardware::broker::scope::{admits, Seen};
use crate::hardware::broker::{table, DeviceRecord};

/// Whether some service `pid`'s process owns may claim `device_id`. A thread
/// is held to its process's rows; a pid with no registered service, or a
/// device not in the table, is refused.
pub(super) fn admitted(pid: u32, device_id: u64) -> bool {
    let Some(record) = table::list().into_iter().find(|r| r.device_id == device_id) else {
        return false;
    };
    let group = crate::process::with_process(pid, |pcb| pcb.thread_group_id())
        .filter(|&tgid| tgid != 0)
        .unwrap_or(pid);
    let seen = seen(&record);
    let served = crate::services::registry::endpoints_of(group);
    if served.iter().any(|ep| admits(&ep.name, &seen)) {
        return true;
    }
    crate::sys::serial::println(b"[CLAIM] refused: the device is outside the driver's scope");
    false
}

fn seen(r: &DeviceRecord) -> Seen {
    Seen {
        bus: r.bus_kind,
        vendor: r.vendor,
        device: r.device,
        class: r.class,
        pci_class: r.pci_class,
        pci_sub: r.pci_subclass,
        progif: r.pci_progif,
    }
}
