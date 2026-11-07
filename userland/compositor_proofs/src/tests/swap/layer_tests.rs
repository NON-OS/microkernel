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

//! OP_PRESENT_LAYER: a present that names its layer by the buffer the
//! layer shows, for a client with a window per layer in one band (the
//! Wayland bridge, every guest of a family under one pid).

use crate::damage::Rect;
use crate::requests::present::{present as decode, present_layer as decode_layer};
use crate::scene::SceneTable;
use crate::scene_submit::submit_keyed_layer;
use crate::state::present_rule::{present, present_on};
use crate::swapchain::present::{on_layer, Present, PRESENT_LAYER_LEN, PRESENT_LEN};
use crate::swapchain::swapchain_damage::Damage;
use crate::tests::scene::multi_rig::keyed;

const GUEST: u32 = 40;
const OTHER: u32 = 41;
const APP_Z: u32 = 2;

fn two_windows() -> SceneTable {
    let mut scene = SceneTable::new();
    submit_keyed_layer(&mut scene, keyed(GUEST, 1, 401, (0, 0, 80, 60)), None).expect("room");
    submit_keyed_layer(&mut scene, keyed(GUEST, 2, 402, (100, 50, 80, 60)), None).expect("room");
    scene
}

fn handles(scene: &SceneTable) -> alloc::vec::Vec<(u32, u64)> {
    scene.layers().map(|l| (l.window_id, l.surface_handle)).collect()
}

fn body(handle: u64, damage: (u32, u32, u32, u32), on: u64) -> [u8; PRESENT_LAYER_LEN] {
    let (x, y, width, height) = damage;
    let damage = Damage { x, y, width, height };
    let p = Present { handle, z: APP_Z, index: 1, damage, opaque: true };
    on_layer(&p.encode(), on)
}

#[test]
fn the_body_the_client_builds_is_the_one_the_compositor_reads() {
    let b = body(0x1_0000_0501, (3, 4, 5, 6), 0x1_0000_0401);
    let (shown, on) = decode_layer(&b).expect("decodes");
    assert_eq!(on, 0x1_0000_0401);
    assert_eq!((shown.handle, shown.z, shown.index, shown.opaque), (0x1_0000_0501, APP_Z, 1, true));
    let a = shown.area;
    assert_eq!((a.x, a.y, a.width, a.height), (3, 4, 5, 6));
    // Its first 32 bytes are a plain present, and read as one.
    assert_eq!(decode(&b[..PRESENT_LEN]), Some(shown));
}

#[test]
fn a_body_of_another_length_or_naming_no_layer_is_refused() {
    let b = body(501, (0, 0, 1, 1), 401);
    assert!(decode_layer(&b[..PRESENT_LEN]).is_none(), "a plain present is not one");
    let mut long = alloc::vec::Vec::from(&b[..]);
    long.push(0);
    assert!(decode_layer(&long).is_none(), "one byte over");
    assert!(decode_layer(&body(501, (0, 0, 1, 1), 0)).is_none(), "no layer named");
    assert!(decode_layer(&body(0, (0, 0, 1, 1), 401)).is_none(), "no buffer");
}

#[test]
fn each_window_takes_its_own_next_buffer_where_a_plain_present_cannot_tell() {
    let mut scene = two_windows();
    let changed = Rect { x: 2, y: 2, width: 4, height: 4 };
    // A plain present of a new buffer cannot tell the guest's two windows apart.
    assert!(present(&mut scene, GUEST, APP_Z, 502, changed).is_none());
    let s = present_on(&mut scene, GUEST, APP_Z, 402, 502, changed).expect("names its layer");
    assert_eq!(s.old, Some(402));
    let r = s.repaint.expect("on the layer");
    assert_eq!((r.x, r.y, r.width, r.height), (102, 52, 4, 4));
    let s = present_on(&mut scene, GUEST, APP_Z, 401, 501, changed).expect("the other");
    assert_eq!(s.old, Some(401));
    assert_eq!(handles(&scene), alloc::vec![(1, 501), (2, 502)]);
    // And back: the buffer each showed first is free to show again.
    let s = present_on(&mut scene, GUEST, APP_Z, 502, 402, changed).expect("back");
    assert_eq!(s.old, Some(502));
    assert_eq!(handles(&scene), alloc::vec![(1, 501), (2, 402)]);
}

#[test]
fn the_same_buffer_again_repaints_and_retires_nothing() {
    let mut scene = two_windows();
    let changed = Rect { x: 0, y: 0, width: 200, height: 200 };
    let s = present_on(&mut scene, GUEST, APP_Z, 401, 401, changed).expect("same buffer");
    assert_eq!(s.old, None);
    let r = s.repaint.expect("cut to the layer");
    assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 80, 60));
}

#[test]
fn a_layer_not_the_senders_or_not_in_the_band_or_a_buffer_another_shows_is_refused() {
    let mut scene = two_windows();
    let before = handles(&scene);
    let changed = Rect { x: 0, y: 0, width: 4, height: 4 };
    assert!(present_on(&mut scene, OTHER, APP_Z, 401, 501, changed).is_none(), "not its layer");
    assert!(present_on(&mut scene, GUEST, APP_Z + 1, 401, 501, changed).is_none(), "other band");
    assert!(present_on(&mut scene, GUEST, APP_Z, 499, 501, changed).is_none(), "no such layer");
    // Window 1's picture is never put in window 2's place.
    assert!(present_on(&mut scene, GUEST, APP_Z, 401, 402, changed).is_none(), "shown by 2");
    assert_eq!(handles(&scene), before, "a refusal moves nothing");
}

#[test]
fn the_op_is_the_first_after_output_rects() {
    let ops = include_str!("../../../../compositor/src/protocol/ops.rs");
    assert!(ops.contains("pub const OP_PRESENT_LAYER: u16 = 0x0022;"));
    assert_eq!(crate::swapchain::present::OP_PRESENT_LAYER, 0x0022);
    assert_eq!(crate::protocol::OP_PRESENT_LAYER, 0x0022);
    assert_eq!(crate::protocol::PRESENT_LAYER_REQ_LEN, PRESENT_LAYER_LEN);
}
