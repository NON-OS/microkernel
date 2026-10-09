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

pub const DESC_CONFIGURATION: u8 = 0x02;
pub const DESC_INTERFACE: u8 = 0x04;
pub const DESC_ENDPOINT: u8 = 0x05;
/// The SuperSpeed Endpoint Companion that follows each endpoint of a USB 3
/// device (USB 3.2 section 9.6.7); byte 2 is bMaxBurst.
pub const DESC_SS_EP_COMPANION: u8 = 0x30;
pub const CLASS_MASS_STORAGE: u8 = 0x08;
pub const SUBCLASS_SCSI_TRANSPARENT: u8 = 0x06;
pub const PROTOCOL_BULK_ONLY: u8 = 0x50;
pub const EP_ATTR_TRANSFER_MASK: u8 = 0x03;
pub const EP_ATTR_BULK: u8 = 0x02;
pub const EP_DIR_IN: u8 = 0x80;
