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

//! One NSO1 message to the core's endpoint and its reply.

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_ipc_call_timeout;

use crate::link::nso1::{message, parse, Head, FRAME_MAX, HEAD};

use super::pipe::ServePipe;

/// How long one read waits before the window draws again.
pub(super) const READ_WAIT_MS: u64 = 25;
/// How long any other message waits for its reply.
pub(super) const CALL_WAIT_MS: u64 = 2000;

impl ServePipe {
    pub(super) fn call(&self, op: u8, data: &[u8], wait: u64) -> Option<(Head, Vec<u8>)> {
        let ask = message(op, self.conn, data);
        let mut buf = vec![0u8; HEAD + FRAME_MAX];
        let n = mk_ipc_call_timeout(
            self.port,
            ask.as_ptr(),
            ask.len(),
            buf.as_mut_ptr(),
            buf.len(),
            wait,
        );
        let (head, body) = parse(buf.get(..usize::try_from(n).ok()?)?)?;
        Some((head, body.to_vec()))
    }
}
