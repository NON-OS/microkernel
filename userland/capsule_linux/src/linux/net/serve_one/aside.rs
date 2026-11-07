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

//! The one Aside the personality keeps (aside_queue.rs): SERVE_ONE's pump
//! puts what is not its own, the input pump takes it before the inbox.

use core::cell::RefCell;

use nonos_app_skeleton::discover::{from_router, from_wm, lookup_service};
use nonos_libc::DELIVERY_LEN;

use super::aside_queue::Aside;

struct Kept(RefCell<Aside>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time; SERVE_ONE's pump and the input pump both
 * run on it, and neither calls the other while it holds the borrow.
 */
unsafe impl Sync for Kept {}

static KEPT: Kept = Kept(RefCell::new(Aside::new()));

/// Keep a frame that owes no answer for the input pump: false when its
/// sender is none the pump believes, it is longer than any frame the pump
/// reads, or the queue is full.
pub(super) fn set_aside(sender: u32, frame: &[u8]) -> bool {
    KEPT.0.borrow_mut().put(sender, frame, DELIVERY_LEN, believed)
}

/// The senders the input pump takes a frame from (wayland/input/pump.rs,
/// control.rs, focus.rs): the input router, the window manager and the
/// desktop shell.
fn believed(sender: u32) -> bool {
    from_router(sender)
        || from_wm(sender)
        || lookup_service(b"desktop_shell").is_some_and(|peer| peer.pid == sender)
}

/// The oldest frame kept, into `into`: its length and sender.
pub fn take_aside(into: &mut [u8]) -> Option<(usize, u32)> {
    KEPT.0.borrow_mut().take(into)
}
