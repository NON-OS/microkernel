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

//! The pure parts of the streams the capsule carries for a guest (its
//! net/routed/), mounted as they ship: the SOCKS endpoint's parser and
//! replies, the retry rule, what a refused Nym open is told, the line a
//! person reads, and the queue of written bytes with the linger that carries
//! it after a close. The rest makes calls to the proxies and is not here.
//!
//! Host double for the table's close (the capsule's routed/io.rs), which
//! the socket table calls when it lets a routed stream go. Each close is
//! recorded for the thread that made it.

use std::cell::RefCell;

#[path = "../../../../../capsule_linux/src/linux/net/routed/line.rs"]
pub mod line;
#[path = "../../../../../capsule_linux/src/linux/net/routed/linger.rs"]
pub mod linger;
#[path = "../../../../../capsule_linux/src/linux/net/routed/nym_errno.rs"]
pub mod nym_errno;
#[path = "../../../../../capsule_linux/src/linux/net/routed/queue.rs"]
pub mod queue;
#[path = "../../../../../capsule_linux/src/linux/net/routed/queue_state.rs"]
mod queue_state;
#[path = "../../../../../capsule_linux/src/linux/net/routed/retry.rs"]
pub mod retry;
#[path = "../../../../../capsule_linux/src/linux/net/routed/socks/mod.rs"]
pub mod socks;

thread_local! {
    static CLOSED: RefCell<Vec<(u32, bool)>> = const { RefCell::new(Vec::new()) };
}

pub fn close(h: u32, abort: bool) {
    CLOSED.with(|c| c.borrow_mut().push((h, abort)));
}

/// The entries closed since the last look, and whether each was aborted.
pub fn closed() -> Vec<(u32, bool)> {
    CLOSED.with(|c| c.borrow_mut().drain(..).collect())
}
