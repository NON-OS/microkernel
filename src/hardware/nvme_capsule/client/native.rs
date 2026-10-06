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

//! One capsule read or write, in the namespace's native LBAs: the capsule's
//! wire carries `lba, count` in LBAs and `count * lba_size` data bytes. The
//! caller has checked the authority and sized the request within one
//! capsule command.

use alloc::vec::Vec;

use super::super::error::DriverNvmeError;
use super::super::protocol::{
    encode_request, MAX_RW_PAYLOAD_BYTES, OP_READ_BLOCKS, OP_WRITE_BLOCKS,
};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

pub(super) fn read_lbas(lba: u64, lbas: u64, out: &mut [u8]) -> Result<(), DriverNvmeError> {
    let count = u32::try_from(lbas).map_err(|_| DriverNvmeError::OversizedRequest)?;
    if count == 0 || out.is_empty() {
        return Err(DriverNvmeError::InvalidArgument);
    }
    if out.len() > MAX_RW_PAYLOAD_BYTES as usize {
        return Err(DriverNvmeError::OversizedRequest);
    }
    let mut body: Vec<u8> = Vec::with_capacity(12);
    body.extend_from_slice(&lba.to_le_bytes());
    body.extend_from_slice(&count.to_le_bytes());
    let request_id = next_request_id();
    let frame = encode_request(OP_READ_BLOCKS, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    if resp.body.len() != out.len() {
        return Err(DriverNvmeError::ProtocolMismatch);
    }
    out.copy_from_slice(&resp.body);
    Ok(())
}

pub(super) fn write_lbas(lba: u64, lbas: u64, data: &[u8]) -> Result<(), DriverNvmeError> {
    let count = u32::try_from(lbas).map_err(|_| DriverNvmeError::OversizedRequest)?;
    if count == 0 || data.is_empty() {
        return Err(DriverNvmeError::InvalidArgument);
    }
    if data.len() > MAX_RW_PAYLOAD_BYTES as usize {
        return Err(DriverNvmeError::OversizedRequest);
    }
    let mut body: Vec<u8> = Vec::with_capacity(12 + data.len());
    body.extend_from_slice(&lba.to_le_bytes());
    body.extend_from_slice(&count.to_le_bytes());
    body.extend_from_slice(data);
    let request_id = next_request_id();
    let frame = encode_request(OP_WRITE_BLOCKS, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status == 0 {
        Ok(())
    } else {
        Err(lift(resp.status))
    }
}
