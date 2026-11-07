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

//! One call to nsh.attest: a frame out, a frame back, and its body when the
//! status is OK. Every length the answer claims is judged against the
//! buffer it came in before anything past the header is read.

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_ipc_call_timeout;
use nsh_front_wire::{parse, write_header, HDR_LEN, ST_OK};

use crate::relay_said::Fault;

/// Send `op` with `body` to `port`, and read the answer into `resp`,
/// waiting at most `wait_ms` (0: the kernel's default, five seconds).
pub fn call(
    port: u64,
    op: u16,
    body: &[u8],
    resp: &mut [u8],
    wait_ms: u64,
) -> Result<Vec<u8>, Fault> {
    let total = HDR_LEN.checked_add(body.len()).ok_or(Fault::Unframed)?;
    let mut req = vec![0u8; total];
    write_header(&mut req, op, ST_OK, body.len()).ok_or(Fault::Unframed)?;
    req[HDR_LEN..].copy_from_slice(body);
    let n =
        mk_ipc_call_timeout(port, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len(), wait_ms);
    if n < 0 {
        return Err(Fault::NoAnswer(n));
    }
    let reply = crate::reply::written(resp, n).ok_or(Fault::Unframed)?;
    let h = parse(reply).ok_or(Fault::Unframed)?;
    if h.op != op {
        return Err(Fault::Unframed);
    }
    match h.status {
        ST_OK => Ok(h.body.to_vec()),
        st => Err(Fault::Status(st)),
    }
}
