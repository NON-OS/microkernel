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

//! Reading a TLS flight off a stream through an anonymity network.
//!
//! The direct reader counts empty socket reads, which is a direct socket's
//! sense of time. Through the Nym mixnet or the Anyone network the first
//! byte of an answer is many seconds out and a reply can arrive in pieces
//! seconds apart, so this reader measures time instead: it waits up to
//! `first_ms` for anything, then up to `quiet_ms` after each piece, and
//! never longer than `total_ms` in all. It stops early once `done` says the
//! flight is whole, or once the far end has finished, which the proxy says.

use alloc::vec::Vec;

pub use super::bounds::Bounds;

/// What a routed reader reads from: `RouteStream` on a machine, a fake in
/// wallet_proofs.
pub trait Source {
    /// What has arrived, waiting up to `wait_ms` for the first of it. Zero
    /// after the wait, or at once when the far end has finished.
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str>;

    fn now_ms(&self) -> i64;
}

/// The bytes of one flight, or None when nothing came, the stream broke
/// before anything did, or more came than a flight may hold.
pub fn read_routed<S: Source>(
    source: &mut S,
    bounds: Bounds,
    done: impl Fn(&[u8]) -> bool,
) -> Option<Vec<u8>> {
    let total = i64::try_from(bounds.total_ms).unwrap_or(i64::MAX);
    let until = source.now_ms().saturating_add(total);
    let mut out = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        let left = until.saturating_sub(source.now_ms()).max(0) as u64;
        let wait = if out.is_empty() { bounds.first_ms } else { bounds.quiet_ms };
        let n = match source.read_wait(&mut chunk, wait.min(left)) {
            Ok(n) => n,
            /* What came before the break is judged as it is. */
            Err(_) => break,
        };
        if n == 0 {
            break;
        }
        out.extend_from_slice(&chunk[..n]);
        if out.len() > bounds.max {
            return None;
        }
        if done(&out) || left == 0 {
            break;
        }
    }
    (!out.is_empty()).then_some(out)
}
