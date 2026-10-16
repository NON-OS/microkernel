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

//! A PCI function that has left: its slot's `_STA` says absent after a
//! bus or device check, or firmware ejected it. Before the broker answers,
//! everything its holder was given is taken back: bus mastering stopped,
//! interrupt routes torn down, DMA grants out of its domain, port grants
//! dropped, MMIO grants dropped and their windows fenced (`mmio::fence`),
//! and the claim with its confining domain released. Then the record and
//! its index handle go, so a later arrival is read afresh under a new id.

use super::super::claim::{lookup, quiesce_held, release};
use super::super::{dma, irq, mmio, pci_index, pio, table};

#[derive(Debug, PartialEq, Eq)]
pub struct Departed {
    pub device_id: u64,
    /// Who held it as it left; that capsule finds its grants gone.
    pub holder: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DepartError {
    /// No function at that address is in the table.
    Unknown,
    /// An eject was asked and this pid holds it; nothing was changed.
    Held(u32),
}

fn revoke(pid: u32, id: u64) -> [usize; 4] {
    let _ = quiesce_held(pid, id);
    let counts = [
        irq::release_for_device(pid, id),
        dma::revoke_for_device(pid, id),
        pio::release_for_device(pid, id),
        mmio::revoke_for_device(pid, id),
    ];
    let _ = release(pid, id);
    counts
}

/// Segment 0 only: it is the only one enumerated. `eject`: firmware is
/// about to be asked to remove it, which waits for its holder to let go;
/// otherwise it is already gone, and the holder loses it.
pub fn depart(bus: u8, dev: u8, func: u8, eject: bool) -> Result<Departed, DepartError> {
    let h = pci_index::all().into_iter().find(|h| {
        let a = h.address;
        a.segment == 0 && (a.bus, a.device, a.function) == (bus, dev, func)
    });
    let id = h.ok_or(DepartError::Unknown)?.device_id;
    let holder = lookup(id).map(|c| c.pid);
    if let (true, Some(pid)) = (eject, holder) {
        return Err(DepartError::Held(pid));
    }
    let counts = holder.map(|pid| revoke(pid, id));
    let _ = table::remove(id);
    let _ = pci_index::take(id);
    super::probe_say::left((bus, dev, func), id, holder.zip(counts));
    Ok(Departed { device_id: id, holder })
}
