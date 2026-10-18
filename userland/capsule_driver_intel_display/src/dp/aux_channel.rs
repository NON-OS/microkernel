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

//! AUX channel transactions (DisplayPort 1.4a section 2.7). A request is
//! a command nibble, a 20-bit address, a length byte (bytes less one) and
//! up to 16 data bytes; a reply opens with a byte whose high nibble says
//! ACK, NACK or DEFER, for native and I2C transactions apart.

pub use super::reply::{i2c_reply, native_reply};

/// Request commands, bits 7:4 of the first request byte.
pub const NATIVE_WRITE: u8 = 0x8;
pub const NATIVE_READ: u8 = 0x9;
pub const I2C_WRITE: u8 = 0x0;
pub const I2C_READ: u8 = 0x1;
/// Middle-of-transaction: the I2C transaction continues after this one.
pub const I2C_MOT: u8 = 0x4;

/// The most data one AUX transaction carries.
pub const MAX_DATA: usize = 16;

/// What the sink answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reply {
    Ack,
    Nack,
    Defer,
}

/// Why an AUX transaction produced no usable answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxError {
    /// The source's transmitter reported no reply in time, or noise.
    Timeout,
    /// The sink NACKed, or kept DEFERring past the retry limit.
    Refused,
    /// A reply too short for what was asked, or a malformed one.
    Short,
}

/// The driver's half: send one request, return the raw reply bytes.
pub trait Aux {
    /// Send `req` (header and data, at most 20 bytes) and put the reply
    /// (its command byte first) into `reply`; the reply's length.
    fn transfer(&mut self, req: &[u8], reply: &mut [u8; 20]) -> Result<usize, AuxError>;
}

/// The four header bytes of a request.
pub fn header(cmd: u8, addr: u32, len: usize) -> [u8; 4] {
    let a = addr & 0xF_FFFF;
    let l = len.clamp(1, MAX_DATA) as u8 - 1;
    [(cmd << 4) | (a >> 16) as u8, (a >> 8) as u8, a as u8, l]
}
