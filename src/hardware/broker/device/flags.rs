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
pub const DEVICE_FLAG_CLAIMED: u32 = 1 << 0;
pub const DEVICE_FLAG_DISABLED: u32 = 1 << 1;
/// Behind a PCIe port firmware calls external-facing: whoever can reach the
/// connector chose what is plugged in, so it gets no untranslated DMA.
pub const DEVICE_FLAG_EXTERNAL: u32 = 1 << 2;
/// A device firmware says depends (_DEP) on one that is missing, dangling
/// or in a cycle: published anyway, so a driver can see it and say why.
pub const DEVICE_FLAG_DEPS_UNMET: u32 = 1 << 3;
/// An ACPI platform device's interrupt as its own `_CRS` describes it:
/// set, edge triggered; clear, level. Meaningful only when the record is
/// an ACPI one with `irq_pin` set; a PCI record never sets it.
pub const DEVICE_FLAG_IRQ_EDGE: u32 = 1 << 4;
/// Likewise: set, active low; clear, active high.
pub const DEVICE_FLAG_IRQ_ACTIVE_LOW: u32 = 1 << 5;
