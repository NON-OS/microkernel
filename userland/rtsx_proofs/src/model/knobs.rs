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

//! What a test turns to make the reader misbehave, and the data phase
//! its SD engine waits on.

/// What a test makes the reader do instead of answering at once.
#[derive(Clone, Copy, Default)]
pub struct Knobs {
    /// Command buffers are taken but never finish.
    pub stall_cmd: bool,
    /// A stop is not instant: a buffer told to stop still finishes, and
    /// writes its results, at the device's next turn (`late`).
    pub late: bool,
    /// A data phase is armed but no data ever moves.
    pub stall_data: bool,
    /// The card leaves the slot the moment a data phase is armed.
    pub pull_on_data: bool,
    /// Every host register reads all ones and takes no write.
    pub gone: bool,
    /// BIPR reads all ones this many more times; then the link is back.
    pub blink: u32,
    /// The data moves, but the transfer ends in an error: a CRC the card
    /// reported on the last block.
    pub data_error: bool,
    /// A transfer that ends in an error raises TRANS_OK, not TRANS_FAIL.
    pub quiet_error: bool,
    /// The card leaves the slot the moment this command is sent, once.
    pub pull_on_cmd: Option<u8>,
    /// An internal register that takes no write: HAIMR answers its old value.
    pub untaken: Option<u16>,
}

/// Data the SD engine waits to move on the next ADMA run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Read,
    Write,
}
