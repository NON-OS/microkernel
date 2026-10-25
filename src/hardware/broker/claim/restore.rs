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

//! A claimed function's configuration read back once it is in D0, and put
//! back where a power transition or reset lost it (L62). Only the BARs
//! enumeration found, never one it found unassigned; decoding is off while
//! they are written and back on after, Bus Master left to the claim.

use super::restore_rule::{holds, put, slots, Kind, Want};
use crate::drivers::pci::constants::{CMD_IO_SPACE, CMD_MEMORY_SPACE};
use crate::drivers::pci::types::PciBar;
use crate::drivers::pci::ConfigSpace;

/// False when a value would not stick: the claim is then refused. Decoding
/// is left as the device has it, as before: its driver turns it on.
pub(super) fn restore(device_id: u64) -> bool {
    let Some(h) = crate::hardware::broker::pci_index::lookup(device_id) else { return true };
    let cfg = ConfigSpace::new(h.address);
    let slots = slots(h.bars.map(|bar| kind_of(&bar)));
    let at = |i: usize| 0x10 + 4 * i as u16;
    let lost = |i: usize, w: Want| !cfg.read32(at(i)).is_ok_and(|r| holds(r, w));
    if !(0..6).any(|i| slots[i].is_some_and(|w| lost(i, w))) {
        return true;
    }
    let Ok(cmd) = cfg.read16(0x04) else { return false };
    let _ = cfg.write16(0x04, cmd & !(CMD_MEMORY_SPACE | CMD_IO_SPACE));
    for (i, w) in slots.iter().enumerate().filter_map(|(i, w)| w.map(|w| (i, w))) {
        let read = cfg.read32(at(i)).unwrap_or(0);
        if !holds(read, w) && (cfg.write32(at(i), put(read, w)).is_err() || lost(i, w)) {
            return false;
        }
    }
    crate::log::info!("[BROKER] device={} lost its BARs on its way to D0; restored", device_id);
    cfg.write16(0x04, cmd).is_ok()
}

fn kind_of(bar: &PciBar) -> (Kind, u64) {
    match *bar {
        PciBar::Memory32 { address, .. } => (Kind::Mem32, address.as_u64()),
        PciBar::Memory64 { address, .. } => (Kind::Mem64, address.as_u64()),
        PciBar::Memory { address, is_64bit: true, .. } => (Kind::Mem64, address.as_u64()),
        PciBar::Memory { address, .. } => (Kind::Mem32, address.as_u64()),
        PciBar::Io { port, .. } => (Kind::Io, port as u64),
        PciBar::NotPresent => (Kind::Absent, 0),
    }
}
