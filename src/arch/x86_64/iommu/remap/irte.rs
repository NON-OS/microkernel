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

//! Remapped-format interrupt remapping table entries (VT-d 3.4, 9.10), low
//! quadword first, laid out as Linux struct irte (include/linux/dmar.h).

pub type Irte = [u64; 2];

/// Where one remapped interrupt goes. Delivery is fixed, to one CPU by its
/// physical APIC id, the way the kernel's own MSI messages ask today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Route {
    pub vector: u8,
    pub destination: u32,
    /// Requester id the unit checks the interrupt against.
    pub source: u16,
    pub level: bool,
}

const PRESENT: u64 = 1 << 0;
const TRIGGER_LEVEL: u64 = 1 << 4;
/// SVT 01 with SQ 00: the request's source id must equal the entry's, all
/// sixteen bits, so no other device can raise this vector (Linux
/// set_irte_sid with SVT_VERIFY_SID_SQ and SQ_ALL_16).
const VERIFY_SOURCE: u64 = 0b01 << 18;

/// The entry for `route`, or `None` when the destination does not fit: an
/// xAPIC-format entry carries eight bits of APIC id in bits 47:40, and a
/// truncated id would deliver to the wrong CPU.
pub const fn encode(route: Route, x2apic: bool) -> Option<Irte> {
    let destination = if x2apic {
        (route.destination as u64) << 32
    } else if route.destination <= 0xFF {
        (route.destination as u64) << 40
    } else {
        return None;
    };
    let trigger = if route.level { TRIGGER_LEVEL } else { 0 };
    let low = PRESENT | trigger | ((route.vector as u64) << 16) | destination;
    Some([low, VERIFY_SOURCE | route.source as u64])
}

pub const fn is_present(entry: Irte) -> bool {
    entry[0] & PRESENT != 0
}

pub const fn vector(entry: Irte) -> u8 {
    (entry[0] >> 16) as u8
}

pub const fn destination(entry: Irte, x2apic: bool) -> u32 {
    if x2apic {
        (entry[0] >> 32) as u32
    } else {
        ((entry[0] >> 40) & 0xFF) as u32
    }
}

pub const fn source(entry: Irte) -> u16 {
    entry[1] as u16
}
