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

//! A mirror's reply read off a stream, within its waits. Pure over the
//! stream, so the host proofs hold it.

use alloc::vec;
use alloc::vec::Vec;

use super::stream::{Reply, Source};

/// How long the first byte of an answer may take.
pub const FIRST_MS: u64 = 120_000;

/// How long a mirror may go quiet once it has started answering.
pub const QUIET_MS: u64 = 120_000;

/// The most one read asks for.
pub const CHUNK: usize = 32 << 10;

pub fn read_reply<S: Source>(source: &mut S, max: usize, done: &dyn Fn(&[u8]) -> bool) -> Reply {
    let mut out = Vec::new();
    let mut chunk = vec![0u8; CHUNK];
    loop {
        let wait = if out.is_empty() { FIRST_MS } else { QUIET_MS };
        let n = match source.read_wait(&mut chunk, wait) {
            Ok(n) => n,
            /* What came before the break is judged as it is. */
            Err(_) => break,
        };
        if n == 0 {
            break;
        }
        out.extend_from_slice(&chunk[..n]);
        if out.len() > max {
            return Reply::TooLarge;
        }
        if done(&out) || source.ended() {
            break;
        }
    }
    match out.is_empty() {
        true => Reply::Nothing,
        false => Reply::Got(out),
    }
}
