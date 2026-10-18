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

//! WDAT read once at boot through the one decoder (`wdat_entry`) and kept
//! for the kernel's watchdog (`arch::x86_64::watchdog::wdat`), which runs
//! its actions through `wdat_run` on the kernel's ports and MMIO.

use super::super::state::TableRegistry;
use super::wdat_decode::Reg;
use super::wdat_entry::{wdat, Wdat};
use super::wdat_run::{run, RunError, WdatBus};
use crate::arch::x86_64::acpi::hw::gas::{gas_read, gas_write, Gas};
use crate::arch::x86_64::acpi::hw::port_bus::PortBus;
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES};
use spin::RwLock;

const SIG_WDAT: u32 = u32::from_le_bytes(*b"WDAT");

static WDAT: RwLock<Option<Wdat>> = RwLock::new(None);

pub fn parse_wdat(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_WDAT) else { return };
    let Some(addr) = super::super::phys::directmap(phys) else { return };
    // SAFETY: as for SRAT and HMAT: the directmap covers what firmware
    // published, and the length is trusted only once the checksum holds.
    let table = unsafe {
        let header = core::ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = u64::from(header.length).min(MAX_TABLE_BYTES) as usize;
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    *WDAT.write() = wdat(table);
}

pub fn wdat_table() -> Option<Wdat> {
    WDAT.read().clone()
}

/// The register from bit 0 up to its field's top; `wdat_run` shifts.
struct Ports;

impl WdatBus for Ports {
    fn read(&mut self, r: &Reg) -> Option<u64> {
        gas_read(&mut PortBus, &whole(r))
    }
    fn write(&mut self, r: &Reg, v: u64) -> bool {
        gas_write(&mut PortBus, &whole(r), v)
    }
}

fn whole(r: &Reg) -> Gas {
    let bit_width = r.bit_width.saturating_add(r.bit_offset);
    Gas { space: r.space, bit_width, bit_offset: 0, access_size: r.access_size, address: r.address }
}

/// Run `action`; `writes` says whether an action that writes may run.
pub fn wdat_action(action: u8, count: u32, writes: bool) -> Result<u64, RunError> {
    let w = WDAT.read();
    run(&w.as_ref().ok_or(RunError::NoAction)?.entries, action, count, writes, &mut Ports)
}
