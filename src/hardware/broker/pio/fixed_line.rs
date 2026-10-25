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

//! The ISA line a channel in compatibility mode interrupts on: 14 for the
//! primary, 15 for the secondary, as the PC/AT wired its two ATA channels
//! (PCI IDE Controller Specification 1.0). Pure, so the proofs run it on the
//! host.

pub(crate) const PRIMARY_IRQ: u32 = 14;
pub(crate) const SECONDARY_IRQ: u32 = 15;

pub(crate) const fn owns_line(primary_fixed: bool, secondary_fixed: bool, irq: u32) -> bool {
    (primary_fixed && irq == PRIMARY_IRQ) || (secondary_fixed && irq == SECONDARY_IRQ)
}
