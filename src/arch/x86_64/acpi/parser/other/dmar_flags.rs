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

//! The DMAR table's flags byte and host address width (VT-d 3.4, 8.1).
//! Bit 0, INTR_REMAP, offers interrupt remapping: clear, the kernel leaves
//! it off. Bit 1, X2APIC_OPT_OUT, asks for xAPIC-format entries, the
//! only format the kernel writes. Bit 2, DMA_CTRL_PLATFORM_OPT_IN_FLAG, is
//! firmware saying it protected memory from DMA through boot and asks the
//! OS to keep it protected: the kernel then never lets firmware's
//! protection go except to its own tables.

use core::sync::atomic::{AtomicU8, Ordering};

const INTR_REMAP: u8 = 1 << 0;
const X2APIC_OPT_OUT: u8 = 1 << 1;
const PLATFORM_OPT_IN: u8 = 1 << 2;

static FLAGS: AtomicU8 = AtomicU8::new(0);
/// HAW + 1 in bits, 0 with no DMAR table.
static WIDTH: AtomicU8 = AtomicU8::new(0);

pub(super) fn record(flags: u8) {
    FLAGS.store(flags, Ordering::Release);
}

/// Firmware set DMA_CTRL_PLATFORM_OPT_IN_FLAG; false with no DMAR table.
pub fn dmar_platform_opt_in() -> bool {
    FLAGS.load(Ordering::Acquire) & PLATFORM_OPT_IN != 0
}

pub(super) fn record_width(haw: u8) {
    WIDTH.store(haw.saturating_add(1), Ordering::Release);
}

/// DMAR offers interrupt remapping (INTR_REMAP); false with no DMAR table.
pub fn dmar_intr_remap() -> bool {
    FLAGS.load(Ordering::Acquire) & INTR_REMAP != 0
}

/// Firmware asks that x2APIC-format remapping stay off (X2APIC_OPT_OUT).
pub fn dmar_x2apic_opt_out() -> bool {
    FLAGS.load(Ordering::Acquire) & X2APIC_OPT_OUT != 0
}

/// The widest DMA address the platform carries, in bits; None without DMAR.
pub fn dmar_address_width() -> Option<u8> {
    Some(WIDTH.load(Ordering::Acquire)).filter(|&w| w != 0)
}
