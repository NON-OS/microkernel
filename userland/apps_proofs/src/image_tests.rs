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

//! The image viewer with nothing to show: a store it could not list is not a
//! store without images, a scan not yet run is neither, and a decoder that is
//! not running is said once, not mistaken for every file being damaged.

use crate::image_budget::{
    refuse_bytes, refuse_pixels, FILE_TOO_LARGE, MAX_IMAGE_BYTES, MAX_PIXELS, READ_LIMIT,
    TOO_MANY_PIXELS,
};
use crate::image_says::{
    decode_reason, decoder_trouble, gallery_next, gallery_note, read_again, rescan_on_press, READ_RETRY_WINDOW_MS, SILENT,
    THUMB_QUIET_MS,
};

/// `VIEWER_HEAP` in the capsule's main.rs.
const VIEWER_HEAP: u64 = 192 * 1024 * 1024;

#[test]
fn a_read_at_the_limit_tells_a_cut_off_file_from_one_that_fits() {
    assert_eq!(READ_LIMIT as u64, MAX_IMAGE_BYTES as u64 + 1);
    assert_eq!(refuse_bytes(MAX_IMAGE_BYTES as u64), None);
    assert_eq!(refuse_bytes(READ_LIMIT as u64), Some(FILE_TOO_LARGE));
    assert_eq!(refuse_bytes(0), None);
}

#[test]
fn an_image_past_the_pixel_budget_is_refused_before_the_copy() {
    assert_eq!(refuse_pixels(5000, 4000), None);
    assert_eq!(refuse_pixels(5000, 4001), Some(TOO_MANY_PIXELS));
    assert_eq!(refuse_pixels(100_000, 100_000), Some(TOO_MANY_PIXELS));
    assert_eq!(refuse_pixels(0, 70_000), None);
}

#[test]
fn the_largest_dimensions_do_not_wrap_past_the_budget() {
    // u32 by u32 overflows u32; the budget multiplies in u64.
    assert_eq!(refuse_pixels(u32::MAX, u32::MAX), Some(TOO_MANY_PIXELS));
    assert_eq!(refuse_pixels(65_536, 65_536), Some(TOO_MANY_PIXELS));
}

#[test]
fn the_budget_fits_the_heap_with_a_rotation_and_the_file_held() {
    let image = MAX_PIXELS * 4;
    let file = u64::from(MAX_IMAGE_BYTES);
    assert!(2 * image + file < VIEWER_HEAP);
}

#[test]
fn a_refused_file_or_image_says_why_in_the_window() {
    assert_eq!(decode_reason(FILE_TOO_LARGE), "the file is larger than 16 MiB");
    assert_eq!(decode_reason(TOO_MANY_PIXELS), "the image is larger than 20 megapixels");
}

#[test]
fn a_store_that_could_not_be_listed_is_not_a_store_without_images() {
    let failed = gallery_note(true, Some("vfs ipc failed"), 0);
    assert_eq!(failed, Some("Images are not available: the file store did not answer"));
    assert_ne!(failed, gallery_note(true, None, 0));
    let refused = gallery_note(true, Some("access denied"), 0);
    assert_eq!(refused, Some("Images are not available: the file store would not list them"));
}

#[test]
fn a_scan_not_yet_run_says_it_is_looking() {
    assert_eq!(gallery_note(false, None, 0), Some("Looking for images..."));
}

#[test]
fn an_empty_store_and_a_full_one_keep_their_words() {
    assert_eq!(gallery_note(true, None, 0), Some("No images found"));
    assert_eq!(gallery_note(true, None, 12), None);
}

#[test]
fn a_decoder_that_is_not_running_is_named_as_such() {
    assert_eq!(decode_reason("no codec"), "the image decoder is not running");
    assert_eq!(decode_reason("codec call failed"), "the image decoder did not answer");
    assert_eq!(decoder_trouble("no codec"), Some("the image decoder is not running"));
    assert_eq!(decoder_trouble("codec call failed"), Some("the image decoder did not answer"));
}

#[test]
fn one_bad_file_is_not_the_decoders_trouble() {
    for err in ["decode error", "no ext", "vfs open failed", "codec dim mismatch"] {
        assert_eq!(decoder_trouble(err), None);
    }
}

#[test]
fn store_and_file_failures_read_as_sentences_and_unknown_ones_pass_through() {
    assert_eq!(decode_reason("vfs ipc failed"), "the file store did not answer");
    assert_eq!(decode_reason("vfs open failed"), "the file could not be read");
    assert_eq!(decode_reason("bad surface geometry"), "bad surface geometry");
}

/* A read refused at once is tried again; eight that each waited out the
 * five-second timeout held the window for forty seconds. */
#[test]
fn a_quick_refusal_is_read_again_and_a_timed_out_read_is_not() {
    assert!(read_again(1, 0, SILENT));
    assert!(read_again(7, READ_RETRY_WINDOW_MS - 1, SILENT));
    assert!(!read_again(1, 5_000, SILENT), "one five-second wait is enough");
    assert!(!read_again(8, 0, SILENT), "eight tries at most");
}

#[test]
fn a_read_the_store_answered_is_not_tried_again() {
    for err in ["vfs open failed", "vfs read failed", "access denied"] {
        assert!(!read_again(1, 0, err), "{err}");
    }
}

/* Thumbnail work rests longer than one silent read takes. */
const _: () = assert!(THUMB_QUIET_MS > 5_000 && READ_RETRY_WINDOW_MS < 5_000);

#[test]
fn an_empty_gallery_says_how_a_picture_gets_there_and_a_press_looks_again() {
    let next = gallery_next(true, None, 0).expect("an empty store says what to do next");
    assert!(next.contains("PNG") && next.contains("Files") && next.contains("press"));
    assert!(rescan_on_press(true, None, 0), "the press the line asks for looks again");
    assert_eq!(gallery_next(true, None, 3), None);
    assert!(!rescan_on_press(true, None, 3), "a gallery with tiles is not walked again");
    assert_eq!(gallery_next(false, None, 0), None, "a scan still running only waits");
    assert!(!rescan_on_press(false, None, 0));
}

#[test]
fn a_store_that_did_not_list_says_a_press_asks_it_again() {
    let next = gallery_next(true, Some("vfs ipc failed"), 0).expect("a way on");
    assert!(next.contains("Press any key"));
    assert!(rescan_on_press(true, Some("vfs ipc failed"), 0));
}
