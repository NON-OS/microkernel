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

//! What a device past the IOMMU may be handed: physical addresses and
//! unremapped interrupts. Pure, proven in kernel_proofs::confine_posture.

use super::decide::opted_in;

/// A device a unit confines can still write 0xFEEx_xxxx: claimed only if
/// interrupts are remapped, under the opt-in, or under the no-IOMMU
/// posture, where an unconfined device can write that page anyway.
pub(crate) const fn interrupts_confined(
    remapping: bool,
    opt_in: bool,
    production: bool,
    posture: bool,
) -> bool {
    remapping || opted_in(opt_in, production) || posture
}

/// Whether a DMA grant may hand a device its physical address: never, but
/// under the opt-in or the no-IOMMU posture.
pub(crate) const fn physical(opt_in_holds: bool, posture: bool) -> bool {
    opt_in_holds || posture
}

/// What a claim that goes ahead unconfined says, by which way it went past.
pub(crate) fn unconfined_words() -> &'static [u8] {
    if crate::memory::iommu::dev_unconfined::holds() {
        b"unconfined under the development opt-in, reaches all memory"
    } else {
        b"no-IOMMU posture: device DMA not confined by hardware, reaches all memory"
    }
}

pub(crate) fn physical_dma() -> bool {
    physical(crate::memory::iommu::dev_unconfined::holds(), crate::memory::iommu::no_iommu::holds())
}
