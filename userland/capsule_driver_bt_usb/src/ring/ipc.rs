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

//! One request to the keyring and its answer, over IPC, bounded in time.

use alloc::{vec, vec::Vec};

use super::keyring::Call;
use nonos_bt_bond::wipe;
use nonos_libc::{mk_ipc_call_timeout, mk_uptime_ms};

/// Sequence, op and two spare octets out; sequence and status back.
const HDR: usize = 8;
/// Record operations take the keyring microseconds; a keyring that does
/// not answer in this long is treated as one that cannot keep bonds.
const TIMEOUT_MS: u64 = 1_000;
const NO_ANSWER: i32 = -11;

pub struct Ipc {
    pub(super) port: u32,
    pub(super) seq: u32,
}

impl Call for Ipc {
    fn call(&mut self, op: u16, payload: &[u8], room: usize) -> Result<Vec<u8>, i32> {
        self.seq = self.seq.wrapping_add(1);
        let mut tx = Vec::with_capacity(HDR + payload.len());
        tx.extend_from_slice(&self.seq.to_le_bytes());
        tx.extend_from_slice(&op.to_le_bytes());
        tx.extend_from_slice(&[0, 0]);
        tx.extend_from_slice(payload);
        let mut rx = vec![0u8; HDR + room];
        let (t, r) = (tx.as_ptr(), rx.as_mut_ptr());
        let n = mk_ipc_call_timeout(self.port.into(), t, tx.len(), r, rx.len(), TIMEOUT_MS);
        wipe(&mut tx);
        // An answer to an earlier request that timed out is not this one's.
        let ours = rx[..4] == self.seq.to_le_bytes();
        let status = rx.get(4..8).map(|s| i32::from_le_bytes([s[0], s[1], s[2], s[3]]));
        let answer = match (usize::try_from(n), status.filter(|_| ours)) {
            (Ok(n), Some(0)) if n >= HDR && n <= rx.len() => Ok(rx[HDR..n].to_vec()),
            (Ok(n), Some(s)) if n >= HDR && s < 0 => Err(s),
            _ => Err(NO_ANSWER),
        };
        wipe(&mut rx);
        answer
    }

    fn now_ms(&mut self) -> u64 {
        mk_uptime_ms().max(0) as u64
    }
}
