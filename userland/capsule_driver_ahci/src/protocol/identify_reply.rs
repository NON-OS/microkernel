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

//! The body of an OP_IDENTIFY reply, after the 4-byte status: what the
//! served disk is, for the installer to show beside its size.
//!
//! ```text
//! offset  size  field
//!      0     8  sectors, u64 little endian (as OP_CAPACITY)
//!      8     4  logical sector size in bytes, u32 little endian (512)
//!     12     1  model length in bytes, 0..=40
//!     13     1  serial length in bytes, 0..=20
//!     14     1  medium: 0 a SATA disk, 1 an eMMC part (this capsule also
//!                   serves eMMC hosts until those get their own capsule)
//!     15     1  reserved, zero
//!     16    40  model, printable ASCII, zero padded
//!     56    20  serial, printable ASCII, zero padded
//! ```
//!
//! The request carries no payload. With no disk served the reply is a bare
//! E_NODEV status.

/// The medium byte: what kind of part the capsule serves.
pub const MEDIUM_SATA: u8 = 0;

pub const IDENTIFY_MODEL_BYTES: usize = 40;
pub const IDENTIFY_SERIAL_BYTES: usize = 20;
pub const IDENTIFY_PAYLOAD_LEN: usize = 16 + IDENTIFY_MODEL_BYTES + IDENTIFY_SERIAL_BYTES;

/// Write the body into `out[..IDENTIFY_PAYLOAD_LEN]`. A model or serial
/// longer than its field is cut to it.
pub fn encode_identify(
    out: &mut [u8],
    sectors: u64,
    sector_bytes: u32,
    model: &[u8],
    serial: &[u8],
    medium: u8,
) {
    let out = &mut out[..IDENTIFY_PAYLOAD_LEN];
    out.fill(0);
    let model = &model[..model.len().min(IDENTIFY_MODEL_BYTES)];
    let serial = &serial[..serial.len().min(IDENTIFY_SERIAL_BYTES)];
    out[0..8].copy_from_slice(&sectors.to_le_bytes());
    out[8..12].copy_from_slice(&sector_bytes.to_le_bytes());
    out[12] = model.len() as u8;
    out[13] = serial.len() as u8;
    out[14] = medium;
    out[16..16 + model.len()].copy_from_slice(model);
    out[56..56 + serial.len()].copy_from_slice(serial);
}
