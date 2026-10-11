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

//! The vendor bits a driver may only turn off. Turned on, they would loosen
//! what the device's DMA does: a relaxed-ordering write may pass the one
//! before it, and a no-snoop one bypasses the CPU caches (claim/no_snoop.rs).

use super::writable::{only, BROADCOM};
use super::Ident;

/// The bits at `offset` a driver of `id` may clear and never set. tg3's chip
/// reset clears Relaxed Ordering (bit 4) and No Snoop (bit 11) in PCI Express
/// Device Control after each core-clock reset (Linux tg3_chip_reset).
#[allow(unreachable_pub)] // reached by kernel_proofs
pub fn clear_only(id: &Ident, offset: u32) -> Option<u16> {
    let devctl = id.pcie_cap.map(|c| c as u32 + 0x08);
    let ethernet = id.class == 0x02 && id.subclass == 0x00;
    (id.vendor == BROADCOM && ethernet && Some(offset) == devctl).then_some(0x0810)
}

/// Whether `new` differs from `current` only by clearing bits of `mask`.
#[allow(unreachable_pub)] // reached by kernel_proofs
pub fn cleared_only(mask: u16, new: u16, current: u16) -> bool {
    only(mask, new, current) && new & mask & !current == 0
}
