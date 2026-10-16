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

//! The body of an IDENTIFY reply, after the status, laid out as the SATA
//! driver lays out its own, so nonos_blk_client reads both with one parse:
//!
//! ```text
//! offset  size  field
//!      0     8  blocks, u64 little endian (as CAPACITY)
//!      8     4  block size in bytes, u32 little endian (512)
//!     12     1  model length in bytes, 0..=40
//!     13     1  serial length in bytes, 0..=20
//!     14     1  medium: 2, an SD card
//!     15     1  reserved, zero
//!     16    40  model: the CID's product name, printable ASCII
//!     56    20  serial: the CID's serial, eight hex digits
//! ```
//!
//! The serial is the word the installer's erase confirmation is typed from;
//! the kernel's own client gets none (C5).

pub const IDENTIFY_LEN: usize = 76;
pub const MEDIUM_SD: u8 = 2;
const MODEL_AT: usize = 16;
const MODEL_BYTES: usize = 40;
const SERIAL_AT: usize = 56;
const SERIAL_BYTES: usize = 20;

/// A model or serial longer than its field is cut to it.
pub fn encode_identify(out: &mut [u8; IDENTIFY_LEN], blocks: u64, model: &[u8], serial: &[u8]) {
    out.fill(0);
    let model = model.get(..MODEL_BYTES).unwrap_or(model);
    let serial = serial.get(..SERIAL_BYTES).unwrap_or(serial);
    out[0..8].copy_from_slice(&blocks.to_le_bytes());
    out[8..12].copy_from_slice(&512u32.to_le_bytes());
    (out[12], out[13], out[14]) = (model.len() as u8, serial.len() as u8, MEDIUM_SD);
    for (dst, &b) in out[MODEL_AT..MODEL_AT + MODEL_BYTES].iter_mut().zip(model) {
        *dst = b;
    }
    for (dst, &b) in out[SERIAL_AT..SERIAL_AT + SERIAL_BYTES].iter_mut().zip(serial) {
        *dst = b;
    }
}
