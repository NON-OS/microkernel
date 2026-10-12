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

pub(in crate::hardware::broker) const BAR_FLAG_PREFETCH: u8 = 1 << 0;
pub(in crate::hardware::broker) const BAR_FLAG_MEM64: u8 = 1 << 1;
// A window the function decodes at a fixed address outside its BARs (an IDE
// channel in compatibility mode, VBoxVGA's DISPI ports), placed in the record
// by the broker rather than read from configuration space.
pub(in crate::hardware::broker) const BAR_FLAG_FIXED: u8 = 1 << 2;
pub const DEVICE_FLAG_CLAIMED: u32 = 1 << 0;
pub const DEVICE_FLAG_DISABLED: u32 = 1 << 1;
/// Behind a PCIe port firmware calls external-facing: whoever can reach the
/// connector chose what is plugged in, so it gets no untranslated DMA.
pub const DEVICE_FLAG_EXTERNAL: u32 = 1 << 2;
/// On an ACPI record with `irq_pin` 1, its `_CRS` interrupt is edge
/// triggered; clear, level (F 51).
pub const DEVICE_FLAG_IRQ_EDGE: u32 = 1 << 4;
/// On an ACPI record with `irq_pin` 1, its `_CRS` interrupt is active low;
/// clear, active high (F 51).
pub const DEVICE_FLAG_IRQ_ACTIVE_LOW: u32 = 1 << 5;
