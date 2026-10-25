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

//! MkAcpiSlot's accesses: one of four registers of a port slot_ports
//! found, and a write only as slot_rule::write_word allows.

use super::slot_ports::{known, port_at};
use super::slot_rule::{write_word, SlotRefusal};
use super::slot_wire::{pack, register, Bdf, DLL_REPORTING, SLOT_CAPS};
use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::types::PciAddress;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotError {
    /// Not a port the kernel lets the capsule drive.
    NotPort,
    /// No such register, or nothing at that index.
    Absent,
    Refused(SlotRefusal),
    /// The configuration access itself failed.
    Platform,
}

pub fn config(f: Bdf) -> ConfigSpace {
    ConfigSpace::new(PciAddress::new(f.1, f.2, f.3))
}

pub fn read(f: Bdf, which: u64) -> Result<u32, SlotError> {
    let p = known(f).ok_or(SlotError::NotPort)?;
    let (at, width) = register(which).ok_or(SlotError::Absent)?;
    let cfg = config(f);
    let got = match width {
        4 => cfg.read32(p.cap + at),
        _ => cfg.read16(p.cap + at).map(u32::from),
    };
    got.map_err(|_| SlotError::Platform)
}

pub fn write(f: Bdf, which: u64, value: u32) -> Result<(), SlotError> {
    let p = known(f).ok_or(SlotError::NotPort)?;
    let (at, _) = register(which).filter(|_| which != SLOT_CAPS).ok_or(SlotError::Absent)?;
    let cfg = config(f);
    let now = cfg.read16(p.cap + at).map_err(|_| SlotError::Platform)?;
    let word = write_word(which, value, now).map_err(SlotError::Refused)?;
    cfg.write16(p.cap + at, word).map_err(|_| SlotError::Platform)
}

pub fn port(n: usize) -> Option<u64> {
    port_at(n).map(|p| pack(p.bdf) | if p.dll { DLL_REPORTING } else { 0 })
}
