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

//! A guest's clipboard selection to the desktop's, and back out of it. A
//! selection set with focus is read and copied (desk_read.rs). A guest that
//! clears its selection, as a password manager does once a copied password
//! has been pasted, clears the desktop's too, but only while it still holds
//! the text this guest put there, so nothing copied since is lost.

use alloc::vec;

use nonos_app_skeleton::clients::clipboard::{clipboard_clear, clipboard_copy, clipboard_paste};

use crate::linux::guest::Guest;

/// The most of the desktop's text compared: the service's largest entry.
const MOST: usize = 64 << 10;

/// The guest set `source` as its clipboard selection, or none.
pub fn copy(guest: &mut Guest, source: Option<u32>) {
    super::desk_read::stop(guest);
    match source {
        Some(source) if super::desk::focused(guest) => super::desk_read::start(guest, source),
        Some(_) => {}
        None => clear(guest),
    }
}

/// A whole selection read: copied, and its digest kept.
pub(super) fn copied(guest: &mut Guest, text: &[u8]) {
    if clipboard_copy(text).is_ok() {
        guest.scene.clip.copied = Some(digest(text));
    }
}

fn clear(guest: &mut Guest) {
    let Some(ours) = guest.scene.clip.copied.take() else { return };
    let mut now = vec![0u8; MOST];
    let n = clipboard_paste(&mut now).unwrap_or(0).min(MOST);
    if n != 0 && digest(&now[..n]) == ours {
        let _ = clipboard_clear();
    }
}

/// FNV-1a, enough to tell the text this guest copied from any other.
fn digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3))
}
