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

//! CONTROL: the command code in byte 0, the length of MESSAGE OUT in byte
//! 1 (zero for every command here), the command's own fields from byte 2
//! (UCSI 1.2, the commands chapter; codes recalled, unverified).

pub const PPM_RESET: u8 = 0x01;
pub const ACK_CC_CI: u8 = 0x04;
pub const SET_NOTIFICATION_ENABLE: u8 = 0x05;
pub const GET_CAPABILITY: u8 = 0x06;
pub const GET_CONNECTOR_CAPABILITY: u8 = 0x07;
pub const GET_CONNECTOR_STATUS: u8 = 0x12;

/// Notifications asked for: command completed (0), external supply (1),
/// power operation mode (2), provider capabilities (5), negotiated power
/// level (6), battery charging status (9), connector partner (11), power
/// direction (12), connect (14), error (15).
pub const NOTIFY: u16 = 0xDA67;

pub fn control(code: u8, fields: u64) -> u64 {
    u64::from(code) | fields << 16
}

pub fn reset() -> u64 {
    control(PPM_RESET, 0)
}

pub fn notifications(mask: u16) -> u64 {
    control(SET_NOTIFICATION_ENABLE, u64::from(mask))
}

pub fn capability() -> u64 {
    control(GET_CAPABILITY, 0)
}

/// Connectors count from 1; seven bits name one.
pub fn connector_capability(n: u8) -> u64 {
    control(GET_CONNECTOR_CAPABILITY, u64::from(n & 0x7F))
}

pub fn connector_status(n: u8) -> u64 {
    control(GET_CONNECTOR_STATUS, u64::from(n & 0x7F))
}

/// Bit 0 acknowledges a connector change, bit 1 a completed command.
pub fn ack(command: bool, change: bool) -> u64 {
    control(ACK_CC_CI, u64::from(change) | u64::from(command) << 1)
}
