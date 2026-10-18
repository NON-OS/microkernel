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

pub const LSI_VENDOR_ID: u16 = 0x1000;
/// The 53C1030 (Ultra320 parallel SCSI): VMware's LSI Logic Parallel,
/// VirtualBox's LsiLogic.
pub const LSI_53C1030_DEVICE_ID: u16 = 0x0030;
/// The SAS1068: VMware's LSI Logic SAS, VirtualBox's LsiLogic SAS, QEMU's
/// mptsas1068.
pub const LSI_SAS1068_DEVICE_ID: u16 = 0x0054;
/// The system interface registers through memory: the first memory BAR
/// that holds them (BAR1 behind the I/O BAR0 on every one of these).
pub const REG_SPACE: u64 = 0x100;
