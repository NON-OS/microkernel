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

//! How many BAR registers a configuration header has (L58). A type 0 header
//! has six. A PCI-to-PCI bridge's has two: the dwords after them hold its bus
//! numbers, its I/O, memory and prefetchable windows and its secondary status.
//! A CardBus bridge's has one, before its socket registers (PCI-to-PCI Bridge
//! 1.2, 3.2; PC Card 8.0; both recalled). Size-probing past them wrote
//! all-ones into a live port's bus numbers and windows, and what read back
//! became a BAR a claimant could map: on a root port, its bus numbers read as
//! a 32-bit window at a low physical address, over ordinary RAM. Pure, so the
//! proofs run it on the host.

/// BAR registers in the header whose type byte (offset 0x0E) is `header`,
/// the multi-function bit aside; none for a type the specification does not
/// define, so nothing is size-probed there.
pub(crate) const fn bar_slots(header: u8) -> u8 {
    match header & 0x7F {
        0x00 => 6,
        0x01 => 2,
        0x02 => 1,
        _ => 0,
    }
}

/// Whether a 64-bit BAR at `index` has its upper half among the header's
/// BARs, so reading it touches no register after them.
pub(crate) const fn upper_half_fits(index: u8, slots: u8) -> bool {
    (index as u16) + 1 < slots as u16
}
