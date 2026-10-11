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

pub const VBOX_VENDOR_ID: u16 = 0x80EE;
pub const VMMDEV_DEVICE_ID: u16 = 0xCAFE;

/// The request port: a request's bus address written here is carried out
/// before the write returns.
pub const PORT_BAR: u8 = 0;
pub const REQUEST_PORT: u16 = 0;
/// VMMDevMemory, the page the host flags pending events in.
pub const MEMORY_BAR: u32 = 1;
pub const MEMORY_MAP_BYTES: u64 = 0x1000;
