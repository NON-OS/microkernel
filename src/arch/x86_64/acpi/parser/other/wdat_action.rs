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

//! WDAT's actions, by the number each instruction names. Recalled from
//! Microsoft's published WDAT description, not read: unverified. RESET
//! restarts the countdown (the kick); the SET actions change the
//! watchdog, the QUERY actions only read it.

pub const RESET: u8 = 0x01;
pub const QUERY_CURRENT_COUNTDOWN: u8 = 0x04;
pub const QUERY_COUNTDOWN: u8 = 0x05;
pub const SET_COUNTDOWN: u8 = 0x06;
pub const QUERY_RUNNING: u8 = 0x08;
pub const SET_RUNNING: u8 = 0x09;
pub const QUERY_STOPPED: u8 = 0x0A;
pub const SET_STOPPED: u8 = 0x0B;
pub const QUERY_REBOOT: u8 = 0x10;
pub const SET_REBOOT: u8 = 0x11;
pub const QUERY_SHUTDOWN: u8 = 0x12;
pub const SET_SHUTDOWN: u8 = 0x13;
pub const QUERY_STATUS: u8 = 0x20;
pub const SET_STATUS: u8 = 0x21;
