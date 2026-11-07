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

//! The desktop shell flips its desk and chrome, each a layer the whole
//! screen wide, between two backings. A frame that changed only the menu
//! bar is presented in place of the shown backing with the bar named
//! (capsule_desktop_shell render/chrome/show.rs): only the bar is composed
//! again, where a submit of the other backing damaged the whole screen
//! every second. A present into a band the layer is not in is refused, and
//! the shell submits whole instead.

use crate::damage::{DamageAccumulator, Rect};
use crate::damage_bound::by_owner;
use crate::protocol::{OP_PRESENT_LAYER, PRESENT_LAYER_REQ_LEN};
use crate::requests::present::{present_layer as decode_layer, INDEX_OPAQUE};
use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::shell_present_body::INDEX_OPAQUE as SHELL_OPAQUE;
use crate::shell_present_body::PRESENT_LAYER_LEN as SHELL_LEN;
use crate::shell_present_body::{present_layer_body, OP_PRESENT_LAYER as SHELL_OP};
use crate::state::present_rule::present_on;
use crate::tests::scene::frame_model::layer;

const SHELL: u32 = 9;
const CHROME_Z: u32 = 3_000_000;
const TRUSTED_Z: u32 = 0x4000_0000;
const DESK_Z: u32 = 1;
const SCREEN: (u32, u32) = (1920, 1080);
const SHOWN: u64 = 0x1_0000_0071;
const NEXT: u64 = 0x1_0000_0072;
const BAR: (u32, u32, u32, u32) = (0, 0, 1920, 28);

fn chrome() -> SceneTable {
    let mut scene = SceneTable::new();
    let mut l = layer(SHELL, 0, 0, SCREEN.0, SCREEN.1, CHROME_Z);
    l.surface_handle = SHOWN;
    submit_layer(&mut scene, l).expect("room");
    scene
}

#[test]
fn the_shells_body_is_the_one_the_compositor_reads() {
    assert_eq!((SHELL_OP, SHELL_LEN), (OP_PRESENT_LAYER, PRESENT_LAYER_REQ_LEN));
    let b = present_layer_body((NEXT, SHOWN), CHROME_Z, (3, 4, 500, 28), false);
    let (shown, on) = decode_layer(&b).expect("decodes");
    assert_eq!(
        (shown.handle, shown.z, shown.index, shown.opaque, on),
        (NEXT, CHROME_Z, 0, false, SHOWN)
    );
    let a = shown.area;
    assert_eq!((a.x, a.y, a.width, a.height), (3, 4, 500, 28));
}

/// The lock screen's curtain is presented as opaque: the compositor reads
/// the shell's bit as its own, and the index stays 0.
#[test]
fn the_curtains_present_reads_as_opaque() {
    assert_eq!(SHELL_OPAQUE, INDEX_OPAQUE);
    let b = present_layer_body((NEXT, SHOWN), TRUSTED_Z, (0, 0, 1920, 1080), true);
    let (shown, on) = decode_layer(&b).expect("decodes");
    assert_eq!(
        (shown.handle, shown.z, shown.index, shown.opaque, on),
        (NEXT, TRUSTED_Z, 0, true, SHOWN)
    );
}

#[test]
fn a_flip_that_changed_the_bar_repaints_the_bar_only() {
    let mut scene = chrome();
    let (x, y, w, h) = BAR;
    let s =
        present_on(&mut scene, SHELL, CHROME_Z, SHOWN, NEXT, Rect { x, y, width: w, height: h });
    let s = s.expect("taken");
    assert_eq!(s.old, Some(SHOWN), "the backing shown until now is held until composed");
    let r = s.repaint.expect("the bar");
    assert_eq!((r.x, r.y, r.width, r.height), BAR);
    let l = scene.layers().find(|l| l.owner_pid == SHELL).expect("the chrome");
    assert_eq!((l.surface_handle, l.x, l.y, l.width, l.height), (NEXT, 0, 0, SCREEN.0, SCREEN.1));
    assert_eq!(scene.layers().count(), 1, "the layer is swapped, not added");
}

#[test]
fn a_flip_into_a_band_the_chrome_is_not_in_is_refused() {
    let mut scene = chrome();
    let (x, y, w, h) = BAR;
    let bar = Rect { x, y, width: w, height: h };
    assert!(present_on(&mut scene, SHELL, TRUSTED_Z, SHOWN, NEXT, bar).is_none());
    assert!(
        present_on(&mut scene, SHELL + 1, CHROME_Z, SHOWN, NEXT, bar).is_none(),
        "not its layer"
    );
    let l = scene.layers().next().expect("the chrome");
    assert_eq!((l.surface_handle, l.z), (SHOWN, CHROME_Z), "left as it was");
}

#[test]
fn a_submit_of_the_other_backing_damaged_the_whole_screen() {
    let mut scene = chrome();
    let mut l = layer(SHELL, 0, 0, SCREEN.0, SCREEN.1, CHROME_Z);
    l.surface_handle = NEXT;
    let rects = submit_layer(&mut scene, l).expect("room");
    let r = rects[0].expect("where it is");
    assert_eq!((r.x, r.y, r.width, r.height), (0, 0, SCREEN.0, SCREEN.1));
}

fn drained(acc: &mut DamageAccumulator) -> Vec<(u32, u32, u32, u32)> {
    let mut out = Vec::new();
    while let Some(r) = acc.drain() {
        out.push((r.x, r.y, r.width, r.height));
    }
    out.sort_unstable();
    out
}

/// The shell's desk and chrome, each shown by the backing `SHOWN`.
fn desk_and_chrome() -> SceneTable {
    let mut scene = chrome();
    let mut desk = layer(SHELL, 0, 0, SCREEN.0, SCREEN.1, DESK_Z);
    desk.surface_handle = SHOWN + 0x10;
    submit_layer(&mut scene, desk).expect("room");
    scene
}

/// What a damage commit of `area` from the shell repaints.
fn committed(scene: &SceneTable, (x, y, w, h): (u32, u32, u32, u32)) -> Vec<(u32, u32, u32, u32)> {
    let mut acc = DamageAccumulator::new();
    by_owner(Rect { x, y, width: w, height: h }, scene.layers(), SHELL)
        .for_each(|(_, r)| acc.accumulate(r));
    drained(&mut acc)
}

#[test]
fn a_commit_after_a_taken_flip_composes_the_bar_a_second_time_and_adds_no_pixel() {
    let mut scene = desk_and_chrome();
    let (x, y, w, h) = BAR;
    let bar = Rect { x, y, width: w, height: h };
    let s = present_on(&mut scene, SHELL, CHROME_Z, SHOWN, NEXT, bar).expect("taken");
    let mut acc = DamageAccumulator::new();
    s.repaint.into_iter().for_each(|r| acc.accumulate(r));
    // The frame after the present composes the bar.
    assert_eq!(drained(&mut acc), [BAR]);
    // The commit the shell sent after it, served once that frame went out,
    // asks for the same bar again: composed twice, and nothing in it the
    // present did not already damage (capsule_desktop_shell
    // render/chrome/sent.rs commits only what no request it took damaged).
    assert_eq!(committed(&scene, BAR), [BAR]);
}

#[test]
fn a_commit_after_a_taken_whole_submit_composes_the_whole_screen_again() {
    let mut scene = desk_and_chrome();
    let mut l = layer(SHELL, 0, 0, SCREEN.0, SCREEN.1, CHROME_Z);
    l.surface_handle = NEXT;
    let mut acc = DamageAccumulator::new();
    submit_layer(&mut scene, l)
        .expect("room")
        .into_iter()
        .flatten()
        .for_each(|r| acc.accumulate(r));
    let whole = (0, 0, SCREEN.0, SCREEN.1);
    assert_eq!(drained(&mut acc), [whole]);
    assert_eq!(committed(&scene, whole), [whole], "the same 2,073,600 pixels again");
}

#[test]
fn a_commit_is_what_shows_a_frame_drawn_in_place() {
    // A single backing drawn on in place is not presented or submitted: the
    // scene does not change, and only the commit repaints the part.
    let scene = desk_and_chrome();
    let before: Vec<_> = scene.layers().map(|l| (l.surface_handle, l.z)).collect();
    assert_eq!(committed(&scene, BAR), [BAR]);
    let after: Vec<_> = scene.layers().map(|l| (l.surface_handle, l.z)).collect();
    assert_eq!(before, after);
}
