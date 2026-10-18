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

pub const AMD_VENDOR_ID: u16 = 0x1022;
/// Am79C970A (PCnet-PCI II, VirtualBox's default and QEMU's `pcnet`) and
/// Am79C973 (PCnet-FAST III) share this id and the programming model;
/// CSR88/89 tell them apart.
pub const PCNET_DEVICE_ID: u16 = 0x2000;
/// BAR1 is the memory-mapped twin of the I/O window in BAR0. Driving the part
/// through it keeps the capsule off port I/O, so it holds no Pio capability.
pub const MMIO_BAR: u32 = 1;
/// APROM, then RDP, RAP, RESET and BDP: the whole word-I/O register file.
pub const WINDOW_BYTES: u64 = 0x20;
