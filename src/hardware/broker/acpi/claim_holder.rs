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

//! Who holds a PCI function's claim, for the ACPI capsule: before it moves
//! a device between D-states for a driver, and when it settles functions
//! it woke at boot. Who is asking is an explicit `Asker`, never a pid
//! value with a meaning of its own.

use super::super::{claim, pci_index};
use super::same_group::same_holder;

/// The process table's thread group for `pid`, for `same_holder`.
pub fn tgid(pid: u32) -> Option<u32> {
    crate::process::with_process(pid, |pcb| pcb.thread_group_id())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asker {
    /// The ACPI capsule's own look: no holder is required, or any will do.
    Anyone,
    /// A driver's request: only the claim's holder.
    Holder(u32),
}

/// Whether the function is claimed (by anyone, or by that holder). None
/// when segment 0 has no such function; only it is enumerated.
pub fn holds(asker: Asker, bdf: (u16, u8, u8, u8)) -> Option<bool> {
    let (seg, bus, dev, func) = bdf;
    let h = pci_index::all().into_iter().find(|h| {
        let a = h.address;
        seg == 0 && a.segment == 0 && (a.bus, a.device, a.function) == (bus, dev, func)
    })?;
    let holder = claim::lookup(h.device_id).map(|c| c.pid);
    Some(match asker {
        Asker::Anyone => holder.is_some(),
        Asker::Holder(pid) => holder.is_some_and(|h| same_holder(h, pid, tgid)),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionError {
    /// No PCI function has that device id.
    NotPci,
    /// The holder named does not hold the device's claim.
    NotHolder,
}

/// The PCI function behind a broker device id: drivers know their device
/// by id, never by bus number.
pub fn function_of_device(
    asker: Asker,
    device_id: u64,
) -> Result<(u16, u8, u8, u8), FunctionError> {
    let h = pci_index::lookup(device_id).ok_or(FunctionError::NotPci)?;
    if let Asker::Holder(pid) = asker {
        if !claim::lookup(device_id).is_some_and(|c| same_holder(c.pid, pid, tgid)) {
            return Err(FunctionError::NotHolder);
        }
    }
    let a = h.address;
    Ok((a.segment, a.bus, a.device, a.function))
}
