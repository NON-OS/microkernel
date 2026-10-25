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

//! CURRENT_STATE: the card state an R1 status reports, and its name.

use super::bits::READY_FOR_DATA;

pub const STATE_IDLE: u8 = 0;
pub const STATE_READY: u8 = 1;
pub const STATE_IDENT: u8 = 2;
pub const STATE_STBY: u8 = 3;
pub const STATE_TRAN: u8 = 4;
pub const STATE_DATA: u8 = 5;
pub const STATE_RCV: u8 = 6;
pub const STATE_PRG: u8 = 7;
pub const STATE_DIS: u8 = 8;

/// CURRENT_STATE, bits 12:9.
pub const fn state(status: u32) -> u8 {
    ((status >> 9) & 0xf) as u8
}

/// The card can take the next data command: ready for data, in transfer
/// state.
pub const fn ready(status: u32) -> bool {
    status & READY_FOR_DATA != 0 && state(status) == STATE_TRAN
}

/// The card state an R1 status reports, by its JEDEC name.
pub const fn state_name(status: u32) -> &'static str {
    match state(status) {
        STATE_IDLE => "idle",
        STATE_READY => "ready",
        STATE_IDENT => "ident",
        STATE_STBY => "stby",
        STATE_TRAN => "tran",
        STATE_DATA => "data",
        STATE_RCV => "rcv",
        STATE_PRG => "prg",
        STATE_DIS => "dis",
        _ => "reserved",
    }
}
