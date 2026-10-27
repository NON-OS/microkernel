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

//! A guest's text selection, read for the desktop clipboard (desk.rs): its
//! source is asked to send into a pipe whose read end the bridge holds, and
//! what it writes, to its end and at most MOST bytes, is copied there.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::clipboard::clipboard_copy;

use crate::linux::call::{close, pipe_end_of, pipe_other_end_open, pipe_pair};
use crate::linux::file::flags::O_CLOEXEC;
use crate::linux::guest::Guest;

use super::mime::text_of;
use super::which::Which;
use super::Event;

/// The most of a selection read; a longer one is not copied at all. The
/// service keeps at most 64 KiB of an entry, and refuses a longer one.
const MOST: usize = 1 << 20;

/// `source` asked to send its text, when it offers any.
pub(super) fn start(guest: &mut Guest, source: u32) {
    let mimes = guest.scene.clip.mimes_of(source);
    let Some(mime) = text_of(&mimes) else { return };
    let Some((slot, fd, write_end)) = pipe_pair(guest, O_CLOEXEC) else { return };
    guest.display.give.push(write_end as u32);
    let send = Which::Data.source_events().0;
    Event::new(source, send).string(mime).send(&mut guest.display.to_client);
    guest.scene.clip.reading = Some(super::state::Reading { slot, fd, bytes: Vec::new() });
}

/// What the source has written so far is taken, and once its end is closed
/// all of it is copied. Run as input is pumped, on each read of the display.
pub fn drain(guest: &mut Guest) {
    let Some(mut r) = guest.scene.clip.reading.take() else { return };
    // A read end the guest closed, or a pipe not lent now, is left be.
    if pipe_end_of(guest, r.fd) != Some((r.slot, false)) {
        guest.scene.clip.reading = guest.pipes.is_empty().then_some(r);
        return;
    }
    r.bytes.append(&mut guest.pipes[r.slot]);
    let whole = (1..=MOST).contains(&r.bytes.len());
    if pipe_other_end_open(guest, r.slot, false) && r.bytes.len() <= MOST {
        guest.scene.clip.reading = Some(r);
        return;
    }
    let _ = close(guest, r.fd);
    if whole {
        let _ = clipboard_copy(&r.bytes);
    }
}

/// A read given up: the selection was replaced or the connection closed.
pub fn stop(guest: &mut Guest) {
    let Some(r) = guest.scene.clip.reading.take() else { return };
    if pipe_end_of(guest, r.fd) == Some((r.slot, false)) {
        let _ = close(guest, r.fd);
    }
}
