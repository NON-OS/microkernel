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

//! The block surface of driver.usb_msc0: the device's size and block
//! length, whole 512-byte sectors in and out, and a flush.

use alloc::vec::Vec;

use super::error::DriverUsbMscError as E;
use super::protocol::*;
use super::transport::round_trip;

/// The device's block count and logical block length.
pub fn geometry() -> Result<(u64, u32), E> {
    let body = round_trip(OP_BLK_CAPACITY, &[])?;
    if body.len() < 12 {
        return Err(E::ProtocolMismatch);
    }
    let blocks = u64::from_le_bytes(body[0..8].try_into().map_err(|_| E::ProtocolMismatch)?);
    let block_len = u32::from_le_bytes(body[8..12].try_into().map_err(|_| E::ProtocolMismatch)?);
    Ok((blocks, block_len))
}

pub fn capacity() -> Result<u64, E> {
    geometry().map(|(blocks, _)| blocks)
}

fn header(lba: u64, bytes: usize) -> Result<Vec<u8>, E> {
    if bytes == 0 || bytes % SECTOR_SIZE != 0 {
        return Err(E::InvalidArgument);
    }
    if bytes > MAX_RW_BYTES {
        return Err(E::OversizedRequest);
    }
    let mut body = Vec::with_capacity(12 + bytes);
    body.extend_from_slice(&lba.to_le_bytes());
    body.extend_from_slice(&((bytes / SECTOR_SIZE) as u32).to_le_bytes());
    Ok(body)
}

/// Read `out.len()` bytes, whole sectors, from `lba`.
pub fn read_blocks(lba: u64, out: &mut [u8]) -> Result<(), E> {
    let body = round_trip(OP_BLK_READ, &header(lba, out.len())?)?;
    if body.len() != out.len() {
        return Err(E::ProtocolMismatch);
    }
    out.copy_from_slice(&body);
    Ok(())
}

/// Write `data`, whole sectors, at `lba`.
pub fn write_blocks(lba: u64, data: &[u8]) -> Result<(), E> {
    let mut body = header(lba, data.len())?;
    body.extend_from_slice(data);
    round_trip(OP_BLK_WRITE, &body).map(|_| ())
}

/// SYNCHRONIZE CACHE on the device.
pub fn flush() -> Result<(), E> {
    round_trip(OP_BLK_FLUSH, &[]).map(|_| ())
}
