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

//! OP_PRESENT_LAYER_PARTS and OP_LAYER_CONTENT_PARTS: a present or a layer's
//! content whose changed box is told apart as up to four boxes inside it.
//! The body the client builds is the one the compositor reads; a block
//! naming a box outside the changed box, none, too many, an empty one or a
//! word past its count is refused; and what it repaints is its boxes alone,
//! kept apart, never the rows between them.

use alloc::vec::Vec;

use crate::damage::{DamageAccumulator, Rect};
use crate::requests::area::Area;
use crate::requests::content::layer_content_parts;
use crate::requests::parts::{parts, Parts};
use crate::requests::present::{present_layer, present_layer_parts};
use crate::scene::{Content, SceneTable};
use crate::scene_submit::submit_keyed_layer;
use crate::state::content_rule::repaint;
use crate::state::present_rule::present_on;
use crate::swapchain::present::{
    on_layer, parts_block, with_parts, Part, Present, MAX_PARTS, OP_PRESENT_LAYER_PARTS, PARTS_LEN,
    PRESENT_LAYER_LEN, PRESENT_LAYER_PARTS_LEN,
};
use crate::swapchain::swapchain_damage::Damage;
use crate::tests::scene::multi_rig::keyed;

const GUEST: u32 = 40;
const APP_Z: u32 = 2;
/// A terminal's window: 1300 by 750 at (300, 100).
const WIN: (u32, u32, u32, u32) = (300, 100, 1300, 750);

fn window() -> SceneTable {
    let mut scene = SceneTable::new();
    submit_keyed_layer(&mut scene, keyed(GUEST, 1, 401, WIN), None).expect("room");
    scene
}

fn body(changed: Part, parts: &[Part]) -> Vec<u8> {
    let (x, y, width, height) = changed;
    let damage = Damage { x, y, width, height };
    let p = Present { handle: 501, z: APP_Z, index: 1, damage, opaque: true };
    let head = on_layer(&p.encode(), 401);
    with_parts(&head, &parts_block(parts).expect("one to four")).to_vec()
}

fn area((x, y, width, height): Part) -> Area {
    Area { x, y, width, height }
}

fn block(words: &[u32]) -> Vec<u8> {
    let mut w = words.to_vec();
    w.resize(PARTS_LEN / 4, 0);
    w.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn listed(p: &Parts) -> Vec<Part> {
    p.iter().map(|a| (a.x, a.y, a.width, a.height)).collect()
}

/// Every box the accumulator holds, drained.
fn drained(acc: &mut DamageAccumulator) -> Vec<(u32, u32, u32, u32)> {
    let mut out = Vec::new();
    while let Some(r) = acc.drain() {
        out.push((r.x, r.y, r.width, r.height));
    }
    out.sort();
    out
}

#[test]
fn the_body_the_client_builds_is_the_one_the_compositor_reads() {
    let rows = [(0, 40, 1300, 20), (0, 720, 1300, 20)];
    let b = body((0, 40, 1300, 700), &rows);
    assert_eq!(b.len(), PRESENT_LAYER_PARTS_LEN);
    let (shown, on, p) = present_layer_parts(&b).expect("decodes");
    assert_eq!((shown.handle, on, shown.z, shown.opaque), (501, 401, APP_Z, true));
    assert_eq!(listed(&p), rows.to_vec());
    // Its head is a plain OP_PRESENT_LAYER body, read as one.
    assert_eq!(present_layer(&b[..PRESENT_LAYER_LEN]), Some((shown, on)));
    assert!(present_layer_parts(&b[..PRESENT_LAYER_LEN]).is_none(), "no block");
    assert!(present_layer_parts(&[b.clone(), alloc::vec![0]].concat()).is_none(), "long");
}

#[test]
fn a_block_names_one_to_four_boxes_each_inside_the_changed_box() {
    let bound = area((10, 20, 100, 50));
    assert_eq!(
        parts(&block(&[1, 10, 20, 100, 50]), bound).map(|p| listed(&p)),
        Some(alloc::vec![(10, 20, 100, 50)])
    );
    let four = [4, 10, 20, 1, 1, 109, 20, 1, 1, 10, 69, 1, 1, 109, 69, 1, 1];
    assert_eq!(parts(&block(&four), bound).map(|p| p.iter().count()), Some(MAX_PARTS));
    assert!(parts(&block(&[0]), bound).is_none(), "none");
    assert!(parts(&block(&[5, 10, 20, 1, 1]), bound).is_none(), "more than four");
    assert!(parts(&block(&[1, 10, 20, 0, 5]), bound).is_none(), "an empty box");
    assert!(parts(&block(&[1, 9, 20, 5, 5]), bound).is_none(), "left of the box");
    assert!(parts(&block(&[1, 10, 19, 5, 5]), bound).is_none(), "above it");
    assert!(parts(&block(&[1, 106, 20, 5, 5]), bound).is_none(), "past its right");
    assert!(parts(&block(&[1, 10, 66, 5, 5]), bound).is_none(), "past its bottom");
    assert!(parts(&block(&[1, u32::MAX, 20, 2, 5]), bound).is_none(), "a right that wraps");
    let wraps = area((u32::MAX - 1, 0, 4, 4));
    assert!(parts(&block(&[1, u32::MAX - 1, 0, 1, 1]), wraps).is_none(), "a bound that wraps");
    assert!(
        parts(&block(&[1, 10, 20, 5, 5, 0, 0, 0, 1]), bound).is_none(),
        "a word past the count"
    );
    assert!(parts(&block(&[1, 10, 20, 5, 5])[..PARTS_LEN - 4], bound).is_none(), "short");
}

#[test]
fn a_block_the_client_cannot_build_is_none() {
    assert!(parts_block(&[]).is_none());
    assert!(parts_block(&[(0, 0, 1, 1); MAX_PARTS + 1]).is_none());
    assert!(parts_block(&[(0, 0, 1, 1); MAX_PARTS]).is_some());
}

/// A selection two rows high near the top of a terminal and its last row:
/// a present of the box round both repaints all the rows between; told
/// apart, only the two.
#[test]
fn two_rows_far_apart_repaint_those_rows_and_not_all_between() {
    let rows = [(0, 40, 1300, 34), (0, 716, 1300, 34)];
    let b = body((0, 40, 1300, 710), &rows);
    let (shown, on, p) = present_layer_parts(&b).expect("decodes");
    let mut scene = window();
    let a = shown.area;
    let changed = Rect { x: a.x, y: a.y, width: a.width, height: a.height };
    let s = present_on(&mut scene, GUEST, APP_Z, on, shown.handle, changed).expect("its layer");
    let mut acc = DamageAccumulator::new();
    for a in p.iter() {
        let part = Rect { x: a.x, y: a.y, width: a.width, height: a.height };
        acc.accumulate(s.repaint_of(part).expect("on the layer"));
    }
    assert_eq!(drained(&mut acc), alloc::vec![(300, 140, 1300, 34), (300, 816, 1300, 34)]);
    // The box alone, as OP_PRESENT_LAYER tells it, is all 710 rows.
    let r = s.repaint.expect("on the layer");
    assert_eq!((r.x, r.y, r.width, r.height), (300, 140, 1300, 710));
    let t = |r: Option<Rect>| r.map(|r| (r.x, r.y, r.width, r.height));
    assert_eq!(t(s.repaint_of(changed)), t(s.repaint), "one part the box is the box");
}

#[test]
fn a_part_is_cut_to_the_layer_as_the_box_is() {
    let mut scene = window();
    let changed = Rect { x: 0, y: 0, width: 4000, height: 4000 };
    let s = present_on(&mut scene, GUEST, APP_Z, 401, 501, changed).expect("its layer");
    let past = Rect { x: 1290, y: 740, width: 100, height: 100 };
    let r = s.repaint_of(past).expect("its corner is on the layer");
    assert_eq!((r.x, r.y, r.width, r.height), (1590, 840, 10, 10));
    assert!(s.repaint_of(Rect { x: 1300, y: 0, width: 5, height: 5 }).is_none(), "off it");
}

#[test]
fn a_contents_parts_are_read_as_named_and_repaint_only_themselves() {
    use crate::requests::content::CONTENT_OPAQUE;
    let head: [u32; 16] = [7, 0, 9, 0, 3, 4, CONTENT_OPAQUE, 0, 40, 1300, 710, 0, 0, 1300, 750, 0];
    let mut b: Vec<u8> = head.iter().flat_map(|v| v.to_le_bytes()).collect();
    b.extend(block(&[2, 0, 40, 1300, 34, 0, 716, 1300, 34]));
    let (set, p) = layer_content_parts(&b).expect("decodes");
    assert_eq!((set.on, set.handle, set.x, set.y, set.opaque), (7, 9, 3, 4, true));
    assert_eq!(listed(&p), alloc::vec![(0, 40, 1300, 34), (0, 716, 1300, 34)]);
    assert!(layer_content_parts(&b[..64]).is_none(), "no block");
    let mut outside = b.clone();
    outside[64 + 8..64 + 12].copy_from_slice(&39u32.to_le_bytes());
    assert!(layer_content_parts(&outside).is_none(), "a part above the box that changed");

    // A first content repaints the whole layer, whatever its parts.
    let mut scene = window();
    let c = Content { handle: 9, x: 0, y: 0, opaque: true, view: (0, 0, 1300, 750) };
    let blank = scene.set_content(GUEST, 401, c).expect("its layer");
    assert_eq!(drained(&mut repainted(&blank, c, &p)), alloc::vec![(300, 100, 1300, 750)]);
    // The same content drawn again: each part alone, kept apart.
    let drawn = scene.set_content(GUEST, 401, c).expect("again");
    let mut acc = repainted(&drawn, c, &p);
    assert_eq!(drained(&mut acc), alloc::vec![(300, 140, 1300, 34), (300, 816, 1300, 34)]);
}

/// What each of `p` repaints as OP_LAYER_CONTENT_PARTS takes it, accumulated.
fn repainted(was: &crate::scene::Layer, c: Content, p: &Parts) -> DamageAccumulator {
    let mut acc = DamageAccumulator::new();
    for a in p.iter() {
        let part = Rect { x: a.x, y: a.y, width: a.width, height: a.height };
        if let Some(r) = repaint(was, c, part) {
            acc.accumulate(r);
        }
    }
    acc
}

#[test]
fn the_ops_follow_layer_content_and_fit_a_request() {
    let ops = include_str!("../../../../compositor/src/protocol/ops.rs");
    assert!(ops.contains("pub const OP_PRESENT_LAYER_PARTS: u16 = 0x0024;"));
    assert!(ops.contains("pub const OP_LAYER_CONTENT_PARTS: u16 = 0x0025;"));
    assert_eq!(OP_PRESENT_LAYER_PARTS, crate::protocol::OP_PRESENT_LAYER_PARTS);
    assert_eq!(PRESENT_LAYER_PARTS_LEN, crate::protocol::PRESENT_LAYER_PARTS_REQ_LEN);
    assert_eq!(PARTS_LEN, crate::protocol::PARTS_LEN);
    assert_eq!(MAX_PARTS, crate::protocol::MAX_PARTS);
    assert!(crate::protocol::LAYER_CONTENT_PARTS_REQ_LEN <= crate::protocol::IPC_PAYLOAD_MAX);
    assert!(PRESENT_LAYER_PARTS_LEN <= crate::protocol::IPC_PAYLOAD_MAX);
}
