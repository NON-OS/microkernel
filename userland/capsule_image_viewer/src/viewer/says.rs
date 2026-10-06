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

//! What the viewer says when it has no image to show. A store that could not
//! be listed is not a store without images, a scan not yet run is neither, and
//! a decoder that is not running is not a damaged file.

/// The gallery's line in place of tiles, or `None` when it has tiles to draw.
pub fn gallery_note(
    scanned: bool,
    scan_error: Option<&'static str>,
    images: usize,
) -> Option<&'static str> {
    if !scanned {
        return Some("Looking for images...");
    }
    match scan_error {
        Some("vfs ipc failed") => Some("Images are not available: the file store did not answer"),
        Some(_) => Some("Images are not available: the file store would not list them"),
        None if images == 0 => Some("No images found"),
        None => None,
    }
}

/// The line under the gallery's note that says what to do next, or None
/// when there is nothing to do but wait. An empty store is the first thing
/// a new session shows: it says how a picture gets here. Both are looked
/// for again on the next key or click (rescan_on_press).
pub fn gallery_next(scanned: bool, scan_error: Option<&'static str>, images: usize) -> Option<&'static str> {
    match (scanned, scan_error, images) {
        (true, None, 0) => Some("Save a PNG, JPEG, BMP or GIF with Files, then press any key here."),
        (true, Some(_), _) => Some("Press any key to ask the file store again."),
        _ => None,
    }
}

/// Why an image could not be shown, worded for the window from the vfs client
/// or decode error behind it; one it does not know is shown as it came.
pub fn decode_reason(err: &'static str) -> &'static str {
    match err {
        "no codec" => "the image decoder is not running",
        "codec call failed" => "the image decoder did not answer",
        "decode error" => "the file is damaged or not an image the decoder reads",
        "vfs ipc failed" => "the file store did not answer",
        "vfs open failed" | "vfs read failed" => "the file could not be read",
        other => other,
    }
}

/// The failures that are the decoder's rather than one file's: every tile
/// fails the same way, so the gallery says it once over the red tiles.
pub fn decoder_trouble(err: &'static str) -> Option<&'static str> {
    match err {
        "no codec" | "codec call failed" => Some(decode_reason(err)),
        _ => None,
    }
}

/// What the vfs client says when the store did not answer at all, after
/// waiting out its five-second reply timeout on the window's thread.
pub const SILENT: &str = "vfs ipc failed";

/// How long after the first try a read the store did not answer is tried
/// again. A call refused at once (the store's queue full) is worth another
/// try; one that waited out the timeout is not, and eight of those held the
/// window for forty seconds.
pub const READ_RETRY_WINDOW_MS: u64 = 1_000;

/// Whether to read again after try number `attempt` (from 1) failed with
/// `err`, `elapsed_ms` after the first began.
pub fn read_again(attempt: u32, elapsed_ms: u64, err: &str) -> bool {
    err == SILENT && attempt < 8 && elapsed_ms < READ_RETRY_WINDOW_MS
}

/// How long thumbnail work rests after a read the store did not answer: one
/// was read per tick, so a silent store cost five seconds a tile, and the
/// tile was marked broken though the file was fine.
pub const THUMB_QUIET_MS: u64 = 10_000;

/// Whether a key or click in the gallery looks again: when the store could
/// not be listed, and when it was listed and held no image. A gallery with
/// tiles is never walked again by a press.
pub fn rescan_on_press(scanned: bool, scan_error: Option<&'static str>, images: usize) -> bool {
    scan_error.is_some() || (scanned && images == 0)
}
