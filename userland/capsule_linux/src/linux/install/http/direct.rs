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

//! A mirror on the local network, dialled directly by address over a socket
//! of this capsule's own (net/raw.rs).

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use super::super::http_reply::{complete, reusable};
use super::get::MAX_BODY;
use crate::linux::net::raw::{connect_host, open_stream_to};
use crate::linux::net::raw_io::{close, recv_until, send_all};

/// How long a mirror may go silent: one fetching upstream before it answers
/// sends nothing for a while, and a slow link for longer under emulation.
const IDLE_MS: u64 = 120_000;

/// The socket kept for the next GET, as keep.rs keeps a route stream: its
/// mirror's address and port, and its handle.
struct Kept(RefCell<Option<(String, u16, u32)>>);

// SAFETY: the install role fetches on one thread, one GET at a time.
unsafe impl Sync for Kept {}

static KEPT: Kept = Kept(RefCell::new(None));

/// The reply from a mirror on the local network, over a socket of this
/// capsule's own, dialled directly: the one the last GET left open if it
/// answers, else a fresh one.
pub(super) fn direct(ip: &str, port: u16, req: &[u8]) -> Option<Vec<u8>> {
    if let Some((at, p, handle)) = KEPT.0.borrow_mut().take() {
        match (at == ip && p == port).then(|| ask(handle, req)).flatten() {
            Some(raw) if !raw.is_empty() => return Some(kept(ip, port, handle, raw)),
            _ => close(handle),
        }
    }
    let handle = open_stream_to(ip)?;
    if connect_host(handle, ip, port).is_none() {
        close(handle);
        return None;
    }
    let Some(raw) = ask(handle, req) else {
        close(handle);
        return None;
    };
    Some(kept(ip, port, handle, raw))
}

fn ask(handle: u32, req: &[u8]) -> Option<Vec<u8>> {
    send_all(handle, req).and_then(|()| recv_until(handle, MAX_BODY, &complete, IDLE_MS))
}

/// `raw`, its socket kept when the reply leaves it fit for another GET.
fn kept(ip: &str, port: u16, handle: u32, raw: Vec<u8>) -> Vec<u8> {
    match reusable(&raw) {
        true => *KEPT.0.borrow_mut() = Some((String::from(ip), port, handle)),
        false => close(handle),
    }
    raw
}
