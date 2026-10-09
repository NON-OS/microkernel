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

//! One OP_RX_BATCH exchange with the NIC driver.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::rx_seq::next_rid;
use crate::protocol::header::{parse_response, write_request};
use crate::protocol::ops::{HDR_LEN, OP_RX_BATCH};

/// A batch of up to 44 frames takes longer to copy than one frame, so its
/// reply is given longer than the 8 ms a single-frame call gets. A reply
/// later than this is not lost: the driver keeps it for the same number.
const BATCH_CALL_MS: u64 = 24;

/// Room for the largest batch the driver sends, and its headers.
const RESP_CAP: usize = HDR_LEN + 4 + 64 * 1024;

/// Ask for batch `seq`, or with `None` only whether batches are served.
/// The batch body on status zero; `None` for any other answer or none.
pub fn call(port: u32, seq: Option<u32>) -> Option<Vec<u8>> {
    let body = seq.map(u32::to_le_bytes);
    let len = body.map_or(0, |b| b.len());
    let mut req = [0u8; HDR_LEN + 4];
    write_request(&mut req, OP_RX_BATCH, next_rid(), len as u32)?;
    if let Some(b) = body {
        req[HDR_LEN..HDR_LEN + 4].copy_from_slice(&b);
    }
    let mut resp = vec![0u8; RESP_CAP];
    let (rp, rl) = (resp.as_mut_ptr(), resp.len());
    let n = mk_ipc_call_timeout(port as u64, req.as_ptr(), HDR_LEN + len, rp, rl, BATCH_CALL_MS);
    let view = resp.get(..usize::try_from(n).ok()?)?;
    let (op, _, _) = parse_response(view)?;
    let status = i32::from_le_bytes(view.get(HDR_LEN..HDR_LEN + 4)?.try_into().ok()?);
    (op == OP_RX_BATCH && status == 0).then(|| view[HDR_LEN + 4..].to_vec())
}
