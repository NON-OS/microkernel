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


//! Reading a mirror's reply off a stream through the network the person
//! chose. Through the Nym mixnet or the Anyone network the first byte of an
//! answer is many seconds out and the rest can come in pieces seconds apart,
//! so the reader measures quiet in time, not in empty reads: it waits up to
//! FIRST_MS for anything, then up to QUIET_MS after each piece. It stops once
//! `done` says the reply is whole, or the far end has finished. Pure, so
//! capsule_linux_proofs drives it with a scripted stream.

use alloc::vec;
use alloc::vec::Vec;

/// How long the first byte of an answer may take.
pub const FIRST_MS: u64 = 120_000;
/// How long a mirror may go quiet once it has started answering.
pub const QUIET_MS: u64 = 120_000;
/// The most one read asks for.
pub const CHUNK: usize = 32 << 10;

/// A stream a reply is read from: `RouteStream` in the capsule, a script in
/// the proofs.
pub trait Source {
    /// What has arrived, waiting up to `wait_ms` for the first of it: zero
    /// when nothing came in that time, or once the far end has finished.
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str>;
    /// The far end finished and all it sent has been read.
    fn ended(&self) -> bool;
}

/// What a read brought.
#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    /// The bytes that came, whole or not; the HTTP framing judges them.
    Got(Vec<u8>),
    /// Nothing came, or the stream broke before anything did.
    Nothing,
    /// More came than `max`: no index or package is that large.
    TooLarge,
}

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

/// Why an install cannot start now, or None when it may: a mirror on the
/// local network is dialled directly whatever the route, and any other is
/// reached only while the network the person chose is running. `down`
/// asks that network for its reason for being down, when it is; it waits up
/// to three minutes for it to come up, so a local mirror never asks.
pub fn offline(local: bool, down: impl FnOnce() -> Option<&'static str>) -> Option<&'static str> {
    match local {
        true => None,
        false => down(),
    }
}
