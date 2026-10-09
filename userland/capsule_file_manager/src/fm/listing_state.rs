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

//! What the listing pane shows when a folder could not be listed. A failed
//! listing is not an empty folder and must not be drawn as one, and the
//! entries already on screen may only stay when they are this folder's own.

/// The empty pane's heading and note. `load_error` is the vfs client's reason
/// the last listing of this folder failed, if it did; `filtered` is whether a
/// name or tag filter is narrowing the view.
pub fn empty_listing(
    load_error: Option<&'static str>,
    filtered: bool,
) -> (&'static str, &'static str) {
    match load_error {
        Some(err) => ("Files are not available", reason(err)),
        None if filtered => ("Nothing here", "No entry matches the current filter."),
        None => ("Nothing here", EMPTY_FOLDER),
    }
}

/// An empty folder, with the keys that put something in it (event_browse.rs:
/// n starts a new file, m a new folder).
pub const EMPTY_FOLDER: &str = "This folder is empty. Press n for a new file, or m for a folder.";

/// A vfs client error as a sentence for the pane; one it does not know is
/// shown as the client worded it rather than hidden.
fn reason(err: &'static str) -> &'static str {
    match err {
        "vfs ipc failed" => "The file store did not answer.",
        "vfs list failed" => "The file store would not list this folder.",
        "vfs list malformed" => "The file store sent a listing that could not be read.",
        "vfs path invalid" => "This folder's path is too long to list.",
        other => other,
    }
}

/// Whether the entries on screen may stay after listing `prefix` failed. They
/// may when they are the listing of this same folder (a refresh that did not
/// land); entries listed for another folder would show that folder's files
/// under this one's name, so they go.
pub fn keep_after_failure(listed: &str, prefix: &str, have_entries: bool) -> bool {
    have_entries && listed == prefix
}

/// What the vfs client says when the store did not answer at all. Each such
/// call waits out the whole reply timeout (five seconds), on the window's
/// thread.
pub const SILENT: &str = "vfs ipc failed";

/// The footer's words for a listing that failed: the pane's note, not the
/// client's code.
pub fn status_line(err: &'static str) -> &'static [u8] {
    reason(err).as_bytes()
}

/// Read each of `keys` with `read`, handing what was read to `take`, and stop
/// at the first read the store did not answer: the next would wait out the
/// same timeout. Opening the window read three settings files and then the
/// folder this way, so a store not answering held it off screen for twenty
/// seconds or more. Returns whether the store went silent.
pub fn read_until_silent<K: Copy, T>(
    keys: &[K],
    mut read: impl FnMut(K) -> Result<T, &'static str>,
    mut take: impl FnMut(K, T),
) -> bool {
    for &key in keys {
        match read(key) {
            Ok(value) => take(key, value),
            Err(SILENT) => return true,
            Err(_) => {}
        }
    }
    false
}
