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

//! What the driver reads from card status: APP_CMD in an R1, the error
//! bits a transfer can end with, whether the card is ready for the next
//! one, and the card's new address in CMD3's R6 (SD Physical Layer 4.9.5
//! and 4.10.1).

/// APP_CMD, bit 5: the card took the last command as CMD55.
const R1_APP_CMD: u32 = 1 << 5;
/// Linux's CMD_ERRORS: out of range, address, block length, write-protect
/// violation, card ECC, card controller and general errors.
const R1_ERRORS: u32 =
    (1 << 31) | (1 << 30) | (1 << 29) | (1 << 26) | (1 << 21) | (1 << 20) | (1 << 19);
const R1_READY_FOR_DATA: u32 = 1 << 8;
/// CURRENT_STATE, bits 12:9, of a card in the transfer state.
const STATE_TRAN: u32 = 4;

pub const fn r1_app_cmd(status: u32) -> bool {
    status & R1_APP_CMD != 0
}

/// The relative card address the card published.
pub const fn r6_rca(r6: u32) -> u16 {
    (r6 >> 16) as u16
}

pub const fn r1_error(status: u32) -> bool {
    status & R1_ERRORS != 0
}

/// Done with what it was sent: ready for data, back in the transfer state.
pub const fn r1_ready(status: u32) -> bool {
    status & R1_READY_FOR_DATA != 0 && (status >> 9) & 0xF == STATE_TRAN
}
