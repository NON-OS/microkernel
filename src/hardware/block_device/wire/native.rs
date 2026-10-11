// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! One READ or WRITE in the disk's own blocks. The caller has sized it
//! within one request.

use alloc::vec::Vec;

use super::error::WireError;
use super::header::{OP_READ, OP_WRITE, RW_HEADER_LEN};
use super::target::Target;
use super::transport::call;

fn head(lba: u64, blocks: u64) -> Result<Vec<u8>, WireError> {
    let count = u32::try_from(blocks).map_err(|_| WireError::OversizedRequest)?;
    if count == 0 {
        return Err(WireError::InvalidArgument);
    }
    let mut body = Vec::with_capacity(RW_HEADER_LEN);
    body.extend_from_slice(&lba.to_le_bytes());
    body.extend_from_slice(&count.to_le_bytes());
    Ok(body)
}

pub(super) fn read_lbas(t: Target, lba: u64, blocks: u64, out: &mut [u8]) -> Result<(), WireError> {
    let body = call(t, OP_READ, &head(lba, blocks)?)?;
    // A short or long reply is a driver that read something else.
    if body.len() != out.len() {
        return Err(WireError::ProtocolMismatch);
    }
    out.copy_from_slice(&body);
    Ok(())
}

pub(super) fn write_lbas(t: Target, lba: u64, blocks: u64, data: &[u8]) -> Result<(), WireError> {
    let mut body = head(lba, blocks)?;
    body.extend_from_slice(data);
    call(t, OP_WRITE, &body).map(|_| ())
}
