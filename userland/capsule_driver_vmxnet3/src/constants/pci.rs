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

pub const VMWARE_VENDOR_ID: u16 = 0x15AD;
pub const VMXNET3_DEVICE_ID: u16 = 0x07B0;
/// BAR0, the pass-through page: interrupt masks and producer doorbells.
pub const PT_BAR: u32 = 0;
/// BAR1, the virtual device page: revision, shared area, commands, MAC.
pub const VD_BAR: u32 = 1;
/// Both pages are 4 KiB on every revision this driver selects.
pub const PT_BYTES: u64 = 0x1000;
pub const VD_BYTES: u64 = 0x1000;
