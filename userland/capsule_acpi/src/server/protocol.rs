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

//! The service's requests, little-endian throughout. Each request is an op
//! byte and its argument; each reply a status word (0, or a negative errno)
//! and the op's body.
//!
//! - STATUS (1): devices u32, routes u32, SCIs u64, GPE runs u64, refused
//!   region accesses u64, armed GPEs u32, notifications dropped u32.
//! - DEVICE (2, index u32): _STA u64, _ADR u64 (all ones if none), _UID u64
//!   (all ones if none), then the _HID's length u8 and bytes, then the
//!   path's length u8 and bytes.

pub const OP_STATUS: u8 = 1;
pub const OP_DEVICE: u8 = 2;
pub const OP_BRIGHTNESS: u8 = 3;
pub const OP_SET_BRIGHTNESS: u8 = 4;
pub const OP_PREPARE_SLEEP: u8 = 5;
pub const OP_POWER: u8 = 6;
pub const OP_THERMAL: u8 = 7;
pub const OP_SMBIOS: u8 = 8;
pub const OP_SERIAL: u8 = 9;
pub const OP_DEVICE_POWER: u8 = 10;
pub const OP_LEGACY: u8 = 11;
pub const OP_PROPERTIES: u8 = 12;
pub const OP_GPIO_EVENT: u8 = 13;
pub const OP_TIME_ALARM: u8 = 14;
pub const OP_TIME_SET: u8 = 15;
pub const OP_QUIRKS: u8 = 16;
pub const OP_LPIT: u8 = 17;
pub const OP_TPM_PPI: u8 = 18;
pub const OP_POWER_METER: u8 = 19;
pub const OP_SET_POWER_CAP: u8 = 20;
pub const OP_HOLDER: u8 = 21;
pub const EPERM: i32 = -1;
pub const ENOENT: i32 = -2;
pub const EIO: i32 = -5;
pub const EAGAIN: i32 = -11;
pub const EINVAL: i32 = -22;
/// Requests and replies fit one IPC message.
pub const MAX: usize = 512;
