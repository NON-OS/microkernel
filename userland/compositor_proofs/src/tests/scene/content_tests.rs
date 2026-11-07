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

//! A layer's content (OP_LAYER_CONTENT): a client's buffer drawn over the
//! layer's own surface from a corner inside it, cut to the layer, in real
//! pixels; who may set it; what it repaints; and a content whose surface is
//! gone, which is never read again.

use super::foreign_desk::*;
use super::foreign_kernel::*;
use crate::damage::Rect;
use crate::scene::{Content, SceneTable};
use crate::state::retire::Retiring;
use crate::tests::scene::frame_util::layer;

const LAYER: (u32, u32, u32, u32) = (8, 8, 40, 30);
const CONTENT: u64 = 5;
/// All of a surface, however large: the draw cuts the box to it.
const ALL: (u32, u32, u32, u32) = (0, 0, u32::MAX, u32::MAX);

fn desk_with_window() -> Desk {
    let mut d = Desk::new();
    d.cursor = (W - 1, H - 1);
    d.kernel.register(2, LAYER.2, LAYER.3, LAYER.2, WINDOW);
    d.submit(2, 2, LAYER, 2);
    d.frame();
    d
}

fn at(d: &Desk, x: u32, y: u32) -> u32 {
    d.canvas[(y * W + x) as usize]
}

fn set(d: &mut Desk, owner: u32, on: u64, c: Content) -> bool {
    let was = d.scene.set_content(owner, on, c);
    if let Some(was) = was {
        let whole = Rect { x: 0, y: 0, width: u32::MAX, height: u32::MAX };
        if let Some(r) = crate::state::content_rule::repaint(&was, c, whole) {
            d.damage.accumulate(r);
        }
    }
    was.is_some()
}

#[test]
fn a_content_is_drawn_at_its_corner_and_cut_to_its_layer() {
    let mut d = desk_with_window();
    d.kernel.register(CONTENT, 80, 50, 80, OTHER);
    assert!(set(&mut d, 2, 2, Content { handle: CONTENT, x: 4, y: 6, opaque: false, view: ALL }));
    d.frame();
    assert_eq!(at(&d, 8 + 4, 8 + 6), OTHER, "its corner");
    assert_eq!(at(&d, 8 + 3, 8 + 6), WINDOW, "left of it, the layer's own");
    assert_eq!(at(&d, 8 + 4, 8 + 5), WINDOW, "above it, the layer's own");
    assert_eq!(at(&d, 8 + 39, 8 + 29), OTHER, "the layer's last pixel");
    assert_eq!(at(&d, 8 + 40, 8 + 29), WALL, "never past the layer");
    assert_eq!(at(&d, 8 + 39, 8 + 30), WALL, "never below it");
    d.assert_clean("content");
}

#[test]
fn an_opaque_content_is_drawn_whatever_its_fourth_byte_and_a_clear_one_shows_what_is_under() {
    let mut d = desk_with_window();
    d.kernel.register(CONTENT, 10, 10, 10, 0x0012_3456);
    assert!(set(&mut d, 2, 2, Content { handle: CONTENT, x: 0, y: 0, opaque: true, view: ALL }));
    d.frame();
    assert_eq!(at(&d, 9, 9), 0xFF12_3456);
    assert!(set(&mut d, 2, 2, Content { handle: CONTENT, x: 0, y: 0, opaque: false, view: ALL }));
    d.frame();
    assert_eq!(at(&d, 9, 9), WINDOW, "alpha 0 blended over the layer is the layer");
}

#[test]
fn a_content_whose_surface_is_gone_is_never_read_again() {
    let mut d = desk_with_window();
    d.kernel.register(CONTENT, 20, 20, 20, OTHER);
    assert!(set(&mut d, 2, 2, Content { handle: CONTENT, x: 2, y: 2, opaque: false, view: ALL }));
    d.frame();
    assert_eq!(at(&d, 12, 12), OTHER);
    // Revoked or its owner gone: the kernel refuses the attach and the
    // frames hold another process's pixels.
    d.kernel.exit(CONTENT);
    d.damage.accumulate(Rect { x: 0, y: 0, width: W, height: H });
    d.frame();
    d.frame();
    d.assert_clean("content gone");
    assert_eq!(at(&d, 12, 12), WINDOW, "the layer's own under it");
    let l = d.scene.layers().find(|l| l.surface_handle == 2).copied().unwrap();
    assert_eq!(l.content, Content::default(), "the layer no longer names it");
    d.staircase("content gone");
}

#[test]
fn a_swapchain_s_next_buffer_is_composed_in_the_box_it_says_changed() {
    const NEXT: u64 = 6;
    const NEW: u32 = 0xFF00_C060;
    let mut d = desk_with_window();
    d.kernel.register(CONTENT, 20, 20, 20, OTHER);
    d.kernel.register(NEXT, 20, 20, 20, NEW);
    let c = Content { handle: CONTENT, x: 2, y: 2, opaque: false, view: ALL };
    assert!(set(&mut d, 2, 2, c));
    d.frame();
    let next = Content { handle: NEXT, ..c };
    let was = d.scene.set_content(2, 2, next).unwrap();
    let box_ = Rect { x: 3, y: 4, width: 5, height: 6 };
    let r = crate::state::content_rule::repaint(&was, next, box_).unwrap();
    assert_eq!((r.x, r.y, r.width, r.height), (8 + 2 + 3, 8 + 2 + 4, 5, 6));
    d.damage.accumulate(r);
    d.frame();
    assert_eq!(at(&d, 13, 14), NEW, "the box, from the next buffer");
    assert_eq!(at(&d, 17, 19), NEW, "its last pixel");
    assert_eq!(at(&d, 18, 14), OTHER, "right of it, left as it was");
    assert_eq!(at(&d, 13, 20), OTHER, "below it, left as it was");
    assert!(set(&mut d, 2, 2, Content { x: 3, ..next }), "moved, it is drawn whole");
    d.frame();
    assert_eq!(at(&d, 8 + 3 + 19, 8 + 2 + 19), NEW);
    d.assert_clean("swapchain");
}

#[test]
fn only_the_layer_s_owner_sets_its_content_and_one_surface_is_one_picture() {
    let mut d = desk_with_window();
    d.kernel.register(3, 10, 10, 10, OTHER);
    d.submit(3, 3, (60, 40, 10, 10), 2);
    let c = Content { handle: CONTENT, x: 0, y: 0, opaque: false, view: ALL };
    assert!(!set(&mut d, 3, 2, c), "another owner's layer");
    assert!(!set(&mut d, 2, 99, c), "a handle no layer of its shows");
    assert!(!set(&mut d, 2, 2, Content { handle: 3, ..c }), "a surface another layer shows");
    assert!(set(&mut d, 2, 2, c));
    assert!(!set(&mut d, 3, 3, c), "a surface another layer draws over its own");
    assert!(set(&mut d, 2, 2, c), "its own content again");
    assert!(set(&mut d, 2, 2, Content::default()));
    assert!(set(&mut d, 3, 3, c), "let go of, it may be another's");
}

#[test]
fn a_move_a_resubmit_and_a_present_keep_the_content() {
    let mut scene = SceneTable::new();
    let mut l = layer(7, 10, 10, 50, 40, 2);
    l.surface_handle = 70;
    scene.submit(l).unwrap();
    let c = Content { handle: 71, x: 3, y: 4, opaque: true, view: ALL };
    assert!(scene.set_content(7, 70, c).is_some());
    scene.submit(super::frame_util::layer(7, 30, 30, 50, 40, 2)).unwrap();
    let moved = scene.layers().find(|l| l.owner_pid == 7).copied().unwrap();
    assert_eq!((moved.x, moved.y, moved.content), (30, 30, c));
    let whole = Rect { x: 0, y: 0, width: 50, height: 40 };
    let shown = crate::state::present_rule::present(&mut scene, 7, 2, 72, whole).unwrap();
    assert_eq!(shown.old, Some(7), "the base swapped");
    let swapped = scene.layers().find(|l| l.owner_pid == 7).copied().unwrap();
    assert_eq!((swapped.surface_handle, swapped.content), (72, c));
}

#[test]
fn a_content_is_held_from_its_release_while_a_layer_draws_it() {
    let mut scene = SceneTable::new();
    let mut l = layer(7, 0, 0, 20, 20, 2);
    l.surface_handle = 70;
    scene.submit(l).unwrap();
    let c = Content { handle: 71, x: 0, y: 0, opaque: false, view: ALL };
    scene.set_content(7, 70, c).unwrap();
    let mut retiring = Retiring::new();
    assert!(retiring.hold(71));
    let drawn =
        |s: &SceneTable, h: u64| s.layers().any(|l| l.surface_handle == h || l.content.handle == h);
    let mut released = Vec::new();
    retiring.release_unshown(|h| drawn(&scene, h), |h| released.push(h));
    assert!(released.is_empty(), "drawn as content, still read");
    scene.set_content(7, 70, Content::default()).unwrap();
    retiring.release_unshown(|h| drawn(&scene, h), |h| released.push(h));
    assert_eq!(released, vec![71]);
}

fn t(r: Option<Rect>) -> Option<(u32, u32, u32, u32)> {
    r.map(|r| (r.x, r.y, r.width, r.height))
}

#[test]
fn a_change_of_content_repaints_the_layer_and_a_redraw_its_box() {
    use crate::state::content_rule::repaint;
    let mut was = layer(7, 100, 50, 40, 30, 2);
    let c = Content { handle: 9, x: 4, y: 2, opaque: false, view: ALL };
    let whole = Some((100, 50, 40, 30));
    assert_eq!(t(repaint(&was, c, Rect { x: 0, y: 0, width: 1, height: 1 })), whole);
    was.content = c;
    let box_ = Rect { x: 1, y: 1, width: 5, height: 5 };
    assert_eq!(t(repaint(&was, c, box_)), Some((105, 53, 5, 5)));
    let past = Rect { x: 30, y: 20, width: 100, height: 100 };
    assert_eq!(t(repaint(&was, c, past)), Some((134, 72, 6, 8)));
    assert_eq!(t(repaint(&was, Content { x: 5, ..c }, box_)), whole, "moved: the whole layer");
    let next = Content { handle: 11, ..c };
    assert_eq!(t(repaint(&was, next, box_)), Some((105, 53, 5, 5)), "a swapchain's next: its box");
    assert_eq!(t(repaint(&was, next, past)), Some((134, 72, 6, 8)), "and cut to the layer");
    let moved = Content { handle: 11, y: 3, ..c };
    assert_eq!(t(repaint(&was, moved, box_)), whole, "another buffer elsewhere: the whole layer");
    let opaque = Content { handle: 11, opaque: true, ..c };
    assert_eq!(t(repaint(&was, opaque, box_)), whole, "drawn another way: the whole layer");
    assert_eq!(t(repaint(&was, Content::default(), box_)), whole, "cleared: the whole layer");
    was.content = Content::default();
    assert_eq!(t(repaint(&was, Content::default(), box_)), None, "nothing drawn, nothing to do");
    assert_eq!(t(repaint(&was, c, box_)), whole, "a first surface: the whole layer");
}

#[test]
fn a_layer_content_body_is_read_only_whole_and_as_named() {
    use crate::requests::content::{layer_content, CONTENT_OPAQUE};
    let body = |w: [u32; 16]| w.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<u8>>();
    let ok = body([7, 0, 9, 1, 3, 4, CONTENT_OPAQUE, 1, 2, 30, 40, 26, 23, 640, 480, 0]);
    let set = layer_content(&ok).unwrap();
    assert_eq!((set.on, set.handle, set.x, set.y, set.opaque), (7, (1 << 32) | 9, 3, 4, true));
    assert_eq!((set.area.x, set.area.y, set.area.width, set.area.height), (1, 2, 30, 40));
    let v = set.view;
    assert_eq!((v.x, v.y, v.width, v.height), (26, 23, 640, 480), "the box of it drawn");
    assert!(layer_content(&ok[..60]).is_none(), "short");
    assert!(layer_content(&ok[..48]).is_none(), "the old twelve words");
    assert!(layer_content(&[ok.clone(), vec![0; 4]].concat()).is_none(), "long");
    assert!(
        layer_content(&body([0, 0, 9, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0])).is_none(),
        "no layer"
    );
    assert!(
        layer_content(&body([9, 0, 9, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0])).is_none(),
        "itself"
    );
    assert!(
        layer_content(&body([7, 0, 9, 0, 0, 0, 2, 0, 0, 1, 1, 0, 0, 1, 1, 0])).is_none(),
        "a flag"
    );
    assert!(
        layer_content(&body([7, 0, 9, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1])).is_none(),
        "padding"
    );
    assert!(
        layer_content(&body([7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0])).is_some(),
        "cleared"
    );
}

#[test]
fn a_layer_draws_only_a_surface_its_owner_answers_for() {
    use crate::state::content_rule::may_show;
    assert!(may_show(Some(7), 7), "its own, or one it pinned from a guest of its");
    assert!(!may_show(Some(8), 7), "another client's");
    assert!(!may_show(None, 7), "one the kernel does not name");
    assert!(!may_show(Some(0), 0), "no process is pid 0");
}

#[test]
fn a_content_s_view_draws_only_that_box_of_it_from_the_corner() {
    let mut d = desk_with_window();
    d.kernel.register(CONTENT, 20, 20, 20, OTHER);
    let c = Content { handle: CONTENT, x: 2, y: 2, opaque: false, view: (5, 5, 8, 6) };
    assert!(set(&mut d, 2, 2, c));
    d.frame();
    assert_eq!(at(&d, 8 + 2, 8 + 2), OTHER, "the box's corner at the content's");
    assert_eq!(at(&d, 8 + 2 + 7, 8 + 2 + 5), OTHER, "its last pixel");
    assert_eq!(at(&d, 8 + 2 + 8, 8 + 2), WINDOW, "right of the box, the layer's own");
    assert_eq!(at(&d, 8 + 2, 8 + 2 + 6), WINDOW, "below it, the layer's own");
    let past = Content { view: (18, 18, 8, 8), ..c };
    assert!(set(&mut d, 2, 2, past));
    d.frame();
    assert_eq!(at(&d, 8 + 2 + 1, 8 + 2 + 1), OTHER, "a box past the edge is cut to the surface");
    assert_eq!(at(&d, 8 + 2 + 2, 8 + 2), WINDOW, "two pixels of it, no more");
    let off = Content { view: (20, 0, 8, 8), ..c };
    assert!(set(&mut d, 2, 2, off));
    d.frame();
    assert_eq!(at(&d, 8 + 2, 8 + 2), WINDOW, "a box off the surface draws nothing");
    d.assert_clean("view");
}

#[test]
fn a_surface_s_view_starts_where_the_box_does_and_ends_with_its_bytes() {
    use crate::sw_blitter::Surface;
    let s = Surface {
        base_va: 0x10_0000,
        stride: 4096,
        width: 1000,
        height: 700,
        byte_len: 4096 * 700,
    };
    let v = s.view((26, 23, 640, 480)).expect("inside");
    assert_eq!(v.base_va, 0x10_0000 + 23 * 4096 + 26 * 4);
    assert_eq!((v.stride, v.width, v.height), (4096, 640, 480));
    assert_eq!(v.byte_len, 4096 * 700 - (23 * 4096 + 26 * 4));
    let last = v.row_start(479, 639, 1).expect("its last pixel");
    assert!(last as u64 + 4 <= s.base_va + s.byte_len, "never past the surface's bytes");
    let cut = s.view((990, 690, 640, 480)).expect("cut to it");
    assert_eq!((cut.width, cut.height), (10, 10));
    assert!(s.view((1000, 0, 1, 1)).is_none(), "right of it");
    assert!(s.view((0, 700, 1, 1)).is_none(), "below it");
    assert!(s.view((0, 0, 0, 5)).is_none(), "empty");
    let short = Surface { byte_len: 23 * 4096, ..s };
    assert!(short.view((26, 23, 640, 480)).is_none(), "a box past the bytes it has");
    let max = Surface { stride: u32::MAX, ..s };
    assert!(max.view((0, 699, 1, 1)).is_none(), "a row start past its bytes, by any stride");
}
