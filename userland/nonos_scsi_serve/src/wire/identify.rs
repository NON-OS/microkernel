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

//! The IDENTIFY reply body: what the disk is called, for the installer to
//! show and to take the erase-confirm word from.
//!
//! ```text
//! offset  size  field
//!      0     1  model_len, 0..=40
//!      1     1  serial_len, 0..=20
//!      2     1  medium: 5 an ATA disk, 6 a SCSI disk
//!      3     1  reserved, zero
//!      4    40  model, printable ASCII, zero padded
//!     44    20  serial, printable ASCII, zero padded
//! ```

pub const IDENTIFY_LEN: usize = 64;
/// An ATA disk behind an IDE controller.
pub const MEDIUM_ATA: u8 = 5;
/// A SCSI disk behind any SCSI host.
pub const MEDIUM_SCSI: u8 = 6;
const MODEL_MAX: usize = 40;
const SERIAL_MAX: usize = 20;
const MODEL_AT: usize = 4;
const SERIAL_AT: usize = 44;

/// The body for `model` and `serial`, their trailing spaces trimmed; None
/// when either is too long or holds a byte outside 0x20..=0x7E, which a
/// client would refuse rather than show.
pub fn encode_identify(model: &[u8], serial: &[u8], medium: u8) -> Option<[u8; IDENTIFY_LEN]> {
    let model = trimmed(model, MODEL_MAX)?;
    let serial = trimmed(serial, SERIAL_MAX)?;
    let mut out = [0u8; IDENTIFY_LEN];
    out[0] = model.len() as u8;
    out[1] = serial.len() as u8;
    out[2] = medium;
    out[MODEL_AT..MODEL_AT + model.len()].copy_from_slice(model);
    out[SERIAL_AT..SERIAL_AT + serial.len()].copy_from_slice(serial);
    Some(out)
}

fn trimmed(field: &[u8], max: usize) -> Option<&[u8]> {
    let end = field.iter().rposition(|&c| c != b' ').map_or(0, |i| i + 1);
    let field = &field[..end];
    (field.len() <= max && field.iter().all(|c| (0x20..=0x7e).contains(c))).then_some(field)
}
