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

//! At late boot, before any device is woken or published: every present
//! PNP0C01 and PNP0C02 device's evaluated _CRS, sent to the kernel in one
//! list. A machine whose namespace declares none sends nothing; the
//! broker then refuses nothing on this account.

use super::wire::{encode, Reservation, IO, MAX_RANGES, MEMORY};
use crate::boot::{evaluator, Acpi};
use crate::sys::log::Line;
use crate::sys::reserved::reserved;
use alloc::vec::Vec;
use nonos_aml::motherboard::{motherboard_ranges, Space};

pub fn publish(acpi: &mut Acpi) {
    let Acpi { aml, host, devices, .. } = acpi;
    let found = motherboard_ranges(aml, &mut evaluator(host), devices);
    if found.is_empty() {
        return Line::new(b"reserved: no motherboard resources declared").end();
    }
    let space = |s: Space| if s == Space::Io { IO } else { MEMORY };
    let list: Vec<Reservation> = found
        .iter()
        .take(MAX_RANGES)
        .map(|r| Reservation { space: space(r.space), base: r.base, len: r.len })
        .collect();
    let io = list.iter().filter(|r| r.space == IO).count() as u64;
    let line = Line::new(b"reserved: ").dec(io).str(b" io, ");
    let line = line.dec(list.len() as u64 - io).str(b" memory");
    let Some(wire) = encode(&list) else { return line.str(b", unencodable").end() };
    match reserved(&wire) {
        Ok(kept) => line.str(b", broker keeps ").dec(kept).end(),
        Err(e) => line.str(b", refused ").dec(e.unsigned_abs()).end(),
    }
}
