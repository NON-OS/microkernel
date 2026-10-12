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

//! Event codes an LE-only host handles (Vol 4, Part E, 7.7), and the LE
//! subevent codes carried inside the LE Meta event (Vol 4, Part E, 7.7.65).

/// Vol 4, Part E, 7.7.5.
pub const DISCONNECTION_COMPLETE: u8 = 0x05;
/// Vol 4, Part E, 7.7.8.
pub const ENCRYPTION_CHANGE: u8 = 0x08;
/// Vol 4, Part E, 7.7.14.
pub const COMMAND_COMPLETE: u8 = 0x0e;
/// Vol 4, Part E, 7.7.15.
pub const COMMAND_STATUS: u8 = 0x0f;
/// Vol 4, Part E, 7.7.16.
pub const HARDWARE_ERROR: u8 = 0x10;
/// Vol 4, Part E, 7.7.19.
pub const NUMBER_OF_COMPLETED_PACKETS: u8 = 0x13;
/// Vol 4, Part E, 7.7.65.
pub const LE_META: u8 = 0x3e;

/// Vol 4, Part E, 7.7.65.1.
pub const LE_CONNECTION_COMPLETE: u8 = 0x01;
/// Vol 4, Part E, 7.7.65.2.
pub const LE_ADVERTISING_REPORT: u8 = 0x02;
/// Vol 4, Part E, 7.7.65.5.
pub const LE_LTK_REQUEST: u8 = 0x05;
