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

//! MkAcpiSlot's answers from enumeration, which replace the capsule's own
//! reads of a bridge's bus numbers: a port's secondary bus, for probing a
//! card that just arrived, and the functions the broker's table holds
//! below it, for a departure.

use super::super::pci_index;
use super::slot_access::{config, SlotError};
use super::slot_ports::known;
use super::slot_rule::below;
use super::slot_wire::{pack, Bdf};

fn buses(f: Bdf) -> Result<(u8, u8), SlotError> {
    known(f).ok_or(SlotError::NotPort)?;
    let cfg = config(f);
    let read = |at| cfg.read8(at).map_err(|_| SlotError::Platform);
    Ok((read(0x19)?, read(0x1A)?))
}

pub fn secondary(f: Bdf) -> Result<u8, SlotError> {
    let (s, _) = buses(f)?;
    below(s, s, s).then_some(s).ok_or(SlotError::Absent)
}

/// The `n`th function in the broker's table below the port, in table order.
pub fn below_at(f: Bdf, n: usize) -> Result<u64, SlotError> {
    let (sec, sub) = buses(f)?;
    let all = pci_index::all();
    let under = all.iter().map(|h| h.address);
    let a = under.filter(|a| a.segment == 0 && below(a.bus, sec, sub)).nth(n);
    let a = a.ok_or(SlotError::Absent)?;
    Ok(pack((0, a.bus, a.device, a.function)))
}
