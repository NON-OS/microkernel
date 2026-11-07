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
//! A line too long for its width is cut with an ellipsis on a character
//! The shell's surfaces kept as two (capsule_desktop_shell state/ctx/
//! flip.rs): a frame is drawn on the backing the compositor is not reading,
//! and shown by submitting it in the shown one's place, so no composite ever
//! reads a frame half written. Booted, one did: the window switcher's strip
//! drawn to half its height over the frame before.

#[path = "../../capsule_desktop_shell/src/state/ctx/flip.rs"]
mod flip;

use flip::Flip;

const FRAME: &str = include_str!("../../capsule_desktop_shell/src/render/chrome/frame.rs");
const SHOW: &str = include_str!("../../capsule_desktop_shell/src/render/chrome/show.rs");

fn two() -> Flip {
    Flip { va: [0x1000, 0x2000], handle: [11, 22], shown: 0 }
}

#[test]
fn surface_flip_draws_where_the_compositor_is_not_reading() {
    let mut f = two();
    assert!(f.flips());
    assert_eq!((f.next_va(), f.next_handle()), (0x2000, 22), "the hidden backing");
    assert_eq!((f.shown_va(), f.shown_handle()), (0x1000, 11));
    f.flipped();
    assert_eq!((f.next_va(), f.next_handle()), (0x1000, 11), "and back");
    assert_eq!((f.shown_va(), f.shown_handle()), (0x2000, 22));
}

#[test]
fn surface_flip_single_draws_in_place() {
    let f = Flip::single(0x3000);
    assert!(!f.flips(), "one backing, no handles");
    assert_eq!(f.next_va(), 0x3000);
    let mut half = two();
    half.handle[1] = 0;
    assert!(!half.flips(), "a backing the compositor never took");
    assert_eq!(half.next_va(), 0x1000, "drawn where it is read, as before");
    let same = Flip { va: [0x1000, 0x1000], handle: [11, 22], shown: 0 };
    assert!(!same.flips(), "two handles on one backing are one surface");
}

#[test]
fn surface_flip_frame_goes_off_screen_and_is_shown_once_taken() {
    assert!(FRAME.contains("(ctx.desk.next_va(), ctx.chrome.next_va())"), "drawn off screen");
    assert!(
        FRAME.contains(
            "let painted = super::show::show(ctx, changed, (desk_new, chrome_new), spans);"
        ),
        "then shown"
    );
    assert!(SHOW.contains("sent(chrome, ctx.chrome.flips(), || flip(ctx, ctx.chrome, (chrome_z, curtain), chrome_at));\n    if chrome.taken() {\n        ctx.chrome.flipped();"));
    // The chrome is presented as opaque only while it is the lock screen's
    // curtain; the desk never is.
    assert!(SHOW.contains("let curtain = ctx.lock.screen.covers();"), "opaque only while locked");
    assert!(SHOW.contains("let desk = sent(desk, ctx.desk.flips(), || flip(ctx, ctx.desk, (DESK_Z, false), desk_at));\n    if desk.taken() {\n        ctx.desk.flipped();"));
}

/// A frame that changed a part is presented in place of the shown backing
/// with that part named, a whole frame with the band of rows it changed
/// (tests/changed_rows_tests.rs); one the compositor refuses, or a whole
/// frame drawn with no frame on screen to hold it to, is submitted whole, and either way the flip counts only once the
/// compositor took it.
#[test]
fn surface_flip_part_is_presented_and_falls_back_to_a_whole_submit() {
    assert!(
        SHOW.contains("(f.next_handle(), f.shown_handle())"),
        "the shown backing names the layer"
    );
    assert!(SHOW.contains("if push_present_layer(port, rid, handles, z, (x, y, w, h), opaque).is_ok() {\n            return Sent::Presented;"));
    assert!(
        SHOW.contains("    if submit(ctx, f.next_handle(), z) {\n        Sent::Submitted"),
        "else submitted whole"
    );
    assert!(
        FRAME.contains("    paint_chrome_in(ctx, Changed::Whole)\n}"),
        "paint_chrome is a whole frame"
    );
}
