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

//! Descriptor types (USB 2.0, table 9-5; CDC 1.2, table 12; USB 3.2,
//! table 9-6) and the class codes the network drivers look for.

pub const DEVICE: u8 = 0x01;
pub const CONFIG: u8 = 0x02;
pub const STRING: u8 = 0x03;
pub const INTERFACE: u8 = 0x04;
pub const ENDPOINT: u8 = 0x05;
/// A class-specific interface descriptor: the CDC functional descriptors.
pub const CS_INTERFACE: u8 = 0x24;
pub const SS_ENDPOINT_COMPANION: u8 = 0x30;

/// bInterfaceClass of a CDC communications interface and of its data one.
pub const CLASS_COMM: u8 = 0x02;
pub const CLASS_CDC_DATA: u8 = 0x0A;
pub const CLASS_VENDOR: u8 = 0xFF;
