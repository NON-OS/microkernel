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

//! One request to the file store and its answer, in the store's own frame
//! (capsule_vfs's protocol): a 20-octet header, "NOVF" version 1, then the
//! payload; the answer repeats the header, then a status word and its data.
//! Each call waits half a second at most, as the policy calls do.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

const MAGIC: u32 = 0x4e4f_5646;
const VERSION: u16 = 1;
const HDR: usize = 20;
const CALL_MS: u64 = 500;

pub struct Call {
    port: u32,
    rx: Vec<u8>,
    id: u32,
}

impl Call {
    pub fn new(port: u32, piece: usize) -> Self {
        Self { port, rx: vec![0; HDR + 4 + piece], id: 0 }
    }
}

/// The answer's data when its status is 0; `None` for a refusal, no
/// answer, or an answer too short to carry a status.
pub fn call<'c>(c: &'c mut Call, op: u16, body: &[u8]) -> Option<&'c [u8]> {
    c.id = c.id.wrapping_add(1);
    let mut tx = Vec::with_capacity(HDR + body.len());
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&[0; 4]);
    tx.extend_from_slice(&c.id.to_le_bytes());
    tx.extend_from_slice(&u32::try_from(body.len()).ok()?.to_le_bytes());
    tx.extend_from_slice(body);
    let (port, rx) = (u64::from(c.port), &mut c.rx);
    let n = mk_ipc_call_timeout(port, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len(), CALL_MS);
    let n = usize::try_from(n).ok().filter(|&n| n >= HDR + 4 && n <= rx.len())?;
    let status = i32::from_le_bytes(rx[HDR..HDR + 4].try_into().ok()?);
    (status == 0).then(|| &rx[HDR + 4..n])
}

/// A service's port and the pid that holds its name, from the registry.
pub fn lookup(name: &[u8]) -> Option<(u32, u32)> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc >= 0 && port != 0 && pid != 0).then_some((port, pid))
}
