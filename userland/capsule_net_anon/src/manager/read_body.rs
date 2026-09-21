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

//! Reading a document until the far end closes.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_uptime_ms, mk_yield};

use crate::tcp_client::recv;

use super::body_closed::finished;
use super::body_step::{step, Step};

/// A consensus is 370 kB on the wire and authorities are often loaded.
const DEADLINE_MS: i64 = 30_000;

const CHUNK: usize = 16 * 1024;

/*
 * The request asks for Connection: close, so the body ends when the far end
 * stops, and net.tcp answers an empty read for both "nothing yet" and
 * "closed". Counting empty reads to tell them apart truncated large documents.
 */

pub(super) fn read_body(tcp_port: u32, handle: u32) -> Option<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    let mut chunk = vec![0u8; CHUNK];
    let deadline = mk_uptime_ms().saturating_add(DEADLINE_MS);
    loop {
        let read = recv(tcp_port, handle, &mut chunk).ok()?;
        let expired = mk_uptime_ms() >= deadline;
        match step(read, read == 0 && finished(tcp_port, handle), expired) {
            Step::Keep => out.extend_from_slice(&chunk[..read]),
            /*
             * Drain before stopping: a peer that has sent its FIN may still
             * have bytes queued here, and stopping on the state alone drops
             * the tail.
             */
            Step::Drain => {
                if recv(tcp_port, handle, &mut chunk).ok()? == 0 {
                    return (!out.is_empty()).then_some(out);
                }
            }
            Step::Stop => {
                return (!out.is_empty()).then_some(out);
            }
            Step::Wait => {
                mk_yield();
            }
        }
    }
}
