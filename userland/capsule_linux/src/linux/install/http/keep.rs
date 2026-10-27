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

//! The stream an install's GETs share, one per mirror (fast downloads, B).
//! Each GET on Anyone opened a stream of its own: an exit's connect and
//! three hops' round trips, seconds each. Now a GET goes out on the stream
//! the last one left, when that reply ended exactly where its length said
//! and the mirror did not say it closes (http_reply.rs, reusable). Any other
//! stream is dropped, and the next GET opens its own on the same route.

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use super::super::http_reply::reusable;
use super::ask::{ask, Done};
use super::stream::Stream;

struct Kept {
    at: String,
    port: u16,
    stream: Stream,
}

struct Slot(RefCell<Option<Kept>>);

// SAFETY: the install role fetches on one thread, one GET at a time.
unsafe impl Sync for Slot {}

static SLOT: Slot = Slot(RefCell::new(None));

/// `request` on the stream kept for `at`:`port`, if one is and it answers.
/// A kept stream the mirror has since closed brings nothing, and the caller
/// sends the GET again on a fresh one.
pub(super) fn again(at: &str, port: u16, req: &[u8], max: usize, done: &Done) -> Option<Vec<u8>> {
    let mut kept = take(at, port)?;
    let got = ask(&mut kept, req, max, done).ok()?;
    keep(at, port, kept, &got);
    Some(got)
}

/// The stream kept for `at`:`port`, if one is; one kept for another mirror
/// is dropped, which closes it.
fn take(at: &str, port: u16) -> Option<Stream> {
    let kept = SLOT.0.borrow_mut().take()?;
    (kept.at == at && kept.port == port).then_some(kept.stream)
}

/// Keep `stream` for the next GET to `at`:`port` when `reply` leaves it
/// fit to carry one; otherwise it is dropped here.
pub(super) fn keep(at: &str, port: u16, stream: Stream, reply: &[u8]) {
    if reusable(reply) && !stream.0.ended() {
        *SLOT.0.borrow_mut() = Some(Kept { at: String::from(at), port, stream });
    }
}
