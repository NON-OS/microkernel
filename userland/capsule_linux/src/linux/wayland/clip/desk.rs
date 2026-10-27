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

//! The desktop clipboard, reached only through the user's own clipboard
//! service, and for text alone (mime.rs). A text selection a guest sets
//! within FOCUS_MS of a key or button its window was given is read by the
//! bridge and copied there (desk_read.rs). A guest holding no clipboard
//! selection of its own is offered the desktop's, and a receive on that
//! offer within FOCUS_MS of such input is written the clipboard's text as
//! it is then. The router sends input only to the window with focus, so a
//! guest in the background neither reads nor replaces the desktop's text.
//! The primary selection never leaves its guest.

use alloc::vec;

use nonos_app_skeleton::clients::clipboard::clipboard_paste;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::call::{family_ms, pipe_end_of};
use crate::linux::guest::Guest;

use super::mime::is_text;
use super::state::DESKTOP;
use super::which::Which;

const FOCUS_MS: u64 = 5_000;
/// The most of the desktop's text written: the service's largest entry.
const MOST_PASTE: usize = 64 << 10;

/// Whether the window was given a key or a button within FOCUS_MS.
pub fn focused(guest: &Guest) -> bool {
    let at = guest.scene.clip.input_ms;
    at != 0 && family_ms().saturating_sub(at) <= FOCUS_MS
}

/// What a clipboard with no guest selection offers: the desktop's, while
/// the service is there. The primary selection has none.
pub fn fallback(which: Which) -> Option<u32> {
    (which == Which::Data && lookup_port(b"clipboard").is_some()).then_some(DESKTOP)
}

/// The guest set `source` as its clipboard selection, or none.
pub fn copy(guest: &mut Guest, source: Option<u32>) {
    super::desk_read::stop(guest);
    if let Some(source) = source.filter(|_| focused(guest)) {
        super::desk_read::start(guest, source);
    }
}

/// A receive on the desktop's offer: its text, put into the pipe the guest
/// passed. The bridge keeps no descriptor on it, and the client closed the
/// one it sent once its request had gone, so its reader sees the text end.
pub fn paste(guest: &mut Guest, mime: &[u8], fd: u64) {
    let Some((slot, true)) = pipe_end_of(guest, fd) else { return };
    if !is_text(mime) || !focused(guest) {
        return;
    }
    let mut text = vec![0u8; MOST_PASTE];
    let n = clipboard_paste(&mut text).unwrap_or(0).min(MOST_PASTE);
    guest.pipes[slot].extend_from_slice(&text[..n]);
}
