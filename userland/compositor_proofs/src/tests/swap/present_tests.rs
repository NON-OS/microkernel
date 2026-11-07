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

use crate::damage::Rect;
use crate::requests::present::present as decode;
use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::state::present_rule::present;
use crate::state::retire::{Retiring, MAX_RETIRING};
use crate::swapchain::present::{Present, PRESENT_LEN};
use crate::swapchain::swapchain_damage::Damage;
use crate::tests::scene::frame_util::layer;

fn desk() -> SceneTable {
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, layer(7, 100, 50, 400, 300, 2)).expect("room");
    submit_layer(&mut scene, layer(8, 0, 0, 1920, 1080, 0)).expect("room");
    scene
}

fn tuple(r: Rect) -> (u32, u32, u32, u32) {
    (r.x, r.y, r.width, r.height)
}

#[test]
fn the_message_the_client_encodes_is_the_one_the_compositor_reads() {
    let p = Present {
        handle: 0x1_0000_0042,
        z: 2,
        index: 1,
        damage: Damage { x: 3, y: 4, width: 50, height: 60 },
        opaque: false,
    };
    let body = p.encode();
    assert_eq!(body.len(), PRESENT_LEN);
    let got = decode(&body).expect("decodes");
    assert_eq!((got.handle, got.z, got.index), (0x1_0000_0042, 2, 1));
    assert_eq!((got.area.x, got.area.y, got.area.width, got.area.height), (3, 4, 50, 60));
    assert!(decode(&body[..31]).is_none(), "a short body");
    assert!(decode(&Present { handle: 0, ..p }.encode()).is_none(), "no handle");
}

#[test]
fn a_present_swaps_the_buffer_and_keeps_the_layer() {
    let mut scene = desk();
    let before = *scene.layers().find(|l| l.owner_pid == 7).expect("layer");
    let s = present(&mut scene, 7, 2, 99, Rect { x: 10, y: 20, width: 30, height: 40 })
        .expect("swapped");
    assert_eq!(s.old, Some(7));
    assert_eq!(s.repaint.map(tuple), Some((110, 70, 30, 40)), "in canvas pixels");
    let after = *scene.layers().find(|l| l.owner_pid == 7).expect("layer");
    assert_eq!(after.surface_handle, 99);
    assert_eq!(
        (after.x, after.y, after.width, after.height),
        (before.x, before.y, before.width, before.height)
    );
    assert_eq!((after.z, after.stack), (before.z, before.stack), "same place in the stack");
}

#[test]
fn a_present_repaints_only_what_is_on_the_layer() {
    let mut scene = desk();
    let s = present(&mut scene, 7, 2, 99, Rect { x: 390, y: 290, width: 50, height: 50 })
        .expect("swapped");
    assert_eq!(s.repaint.map(tuple), Some((490, 340, 10, 10)));
    let s =
        present(&mut scene, 7, 2, 98, Rect { x: 500, y: 0, width: 5, height: 5 }).expect("swapped");
    assert!(s.repaint.is_none(), "past the layer: nothing");
}

#[test]
fn a_present_of_the_buffer_already_shown_holds_nothing() {
    let mut scene = desk();
    let s = present(&mut scene, 7, 2, 7, Rect { x: 0, y: 0, width: 4, height: 4 }).expect("same");
    assert_eq!(s.old, None);
}

#[test]
fn a_present_for_a_layer_the_sender_has_not_is_refused() {
    let mut scene = desk();
    assert!(
        present(&mut scene, 7, 3, 99, Rect { x: 0, y: 0, width: 4, height: 4 }).is_none(),
        "no layer in band 3"
    );
    assert!(
        present(&mut scene, 9, 2, 99, Rect { x: 0, y: 0, width: 4, height: 4 }).is_none(),
        "not its layer"
    );
    assert_eq!(scene.layers().find(|l| l.owner_pid == 7).map(|l| l.surface_handle), Some(7));
}

#[test]
fn held_buffers_are_released_once_no_layer_shows_them() {
    let mut r = Retiring::new();
    assert!(r.hold(5) && r.hold(6) && r.hold(5), "a buffer held twice is held once");
    let mut released = Vec::new();
    r.release_unshown(|h| h == 6, |h| released.push(h));
    assert_eq!(released, [5]);
    released.clear();
    r.release_unshown(|_| false, |h| released.push(h));
    assert_eq!(released, [6]);
    assert!(r.is_empty());
}

#[test]
fn a_full_hold_refuses_rather_than_releasing_early() {
    let mut r = Retiring::new();
    for h in 1..=MAX_RETIRING as u64 {
        assert!(r.hold(h));
    }
    assert!(!r.has_room());
    assert!(!r.hold(1000));
    let mut released = 0;
    r.release_unshown(|_| true, |_| released += 1);
    assert_eq!(released, 0, "every one still shown: none let go");
}

#[test]
fn a_shown_frame_moves_the_count_and_the_time_in_nanoseconds() {
    use crate::state::presented::{Presented, FRAME_TIME_LEN};
    let mut p = Presented::default();
    p.shown(1_234);
    p.shown(1_251);
    let b = p.encode();
    assert_eq!(b.len(), FRAME_TIME_LEN);
    let word = |at: usize| u64::from_le_bytes(b[at..at + 8].try_into().expect("8 bytes"));
    assert_eq!((word(0), word(8), word(16)), (2, 1_251_000_000, 0));
}
