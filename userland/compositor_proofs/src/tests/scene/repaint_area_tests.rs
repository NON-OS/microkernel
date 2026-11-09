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

//! The area each common change repaints, exactly. Touching damage is merged
//! into one rectangle when the box costs at most 64x64 pixels more than
//! the two apart, so a small diagonal move repaints the box round where a
//! thing was and is: that box less the two places is two corners of `dx` by
//! `dy` pixels. Past that, the two places are repainted apart.

use crate::damage::{DamageAccumulator, Rect};
use crate::damage_bound::by_owner;
use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::state::cursor::CursorState;
use crate::state::cursor_pick::{reach, repaint};
use crate::tests::scene::frame_util::layer;

const SCREEN: (u32, u32) = (1920, 1080);

fn drained(acc: &mut DamageAccumulator) -> Vec<(u32, u32, u32, u32)> {
    let mut v: Vec<_> =
        core::iter::from_fn(|| acc.drain()).map(|r| (r.x, r.y, r.width, r.height)).collect();
    v.sort_unstable();
    v
}

fn shown(x: u32, y: u32) -> CursorState {
    CursorState { x, y, visible: true }
}

fn moved(was: CursorState, now: CursorState, plane: bool) -> Vec<(u32, u32, u32, u32)> {
    let mut acc = DamageAccumulator::new();
    repaint(was, now, plane, SCREEN).into_iter().flatten().for_each(|r| acc.accumulate(r));
    drained(&mut acc)
}

fn tuple(r: Rect) -> (u32, u32, u32, u32) {
    (r.x, r.y, r.width, r.height)
}

#[test]
fn a_pointer_step_repaints_the_box_round_its_two_places() {
    let (a, b) = (reach(500, 300, 1920, 1080), reach(508, 304, 1920, 1080));
    let got = moved(shown(500, 300), shown(508, 304), false);
    assert_eq!(got, [tuple(a.union(b))]);
    let area = a.union(b).area();
    assert_eq!(area, 40 * 36);
    let both = a.area() + b.area() - a.intersect(b).map_or(0, |r| r.area());
    assert_eq!(area - both, 2 * 8 * 4, "two corners of 8 by 4");
}

#[test]
fn a_pointer_jump_repaints_its_two_places_only() {
    let got = moved(shown(100, 100), shown(900, 600), false);
    let want = [tuple(reach(100, 100, 1920, 1080)), tuple(reach(900, 600, 1920, 1080))];
    assert_eq!(got, want);
    assert_eq!(got.iter().map(|r| r.2 * r.3).sum::<u32>(), 2 * 32 * 32);
}

#[test]
fn a_straight_pointer_step_repaints_nothing_past_its_two_places() {
    let got = moved(shown(500, 300), shown(512, 300), false);
    assert_eq!(got, [(490, 290, 44, 32)]);
}

#[test]
fn the_cursor_plane_moves_with_no_repaint() {
    assert!(moved(shown(500, 300), shown(508, 304), true).is_empty());
    assert!(moved(shown(5, 5), CursorState { x: 900, y: 9, visible: false }, true).is_empty());
}

#[test]
fn a_hidden_pointer_repaints_only_where_it_shows() {
    let hidden = CursorState { x: 40, y: 40, visible: false };
    assert_eq!(moved(hidden, shown(700, 70), false), [tuple(reach(700, 70, 1920, 1080))]);
    assert_eq!(moved(shown(700, 70), hidden, false), [tuple(reach(700, 70, 1920, 1080))]);
    assert!(moved(hidden, hidden, false).is_empty());
}

#[test]
fn a_caret_blink_repaints_one_glyph_cell() {
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, layer(7, 200, 100, 800, 600, 2)).expect("room");
    let cell = Rect { x: 200 + 10 * 8, y: 100 + 4 * 16, width: 8, height: 16 };
    let mut acc = DamageAccumulator::new();
    by_owner(cell, scene.layers(), 7).for_each(|(_, r)| acc.accumulate(r));
    assert_eq!(drained(&mut acc), [tuple(cell)]);
}

#[test]
fn a_cell_past_the_window_repaints_only_the_windows_part() {
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, layer(7, 200, 100, 800, 600, 2)).expect("room");
    let cell = Rect { x: 996, y: 120, width: 8, height: 16 };
    let mut acc = DamageAccumulator::new();
    by_owner(cell, scene.layers(), 7).for_each(|(_, r)| acc.accumulate(r));
    assert_eq!(drained(&mut acc), [(996, 120, 4, 16)]);
    by_owner(cell, scene.layers(), 8).for_each(|(_, r)| acc.accumulate(r));
    assert!(drained(&mut acc).is_empty(), "another client's damage is not this window's");
}

type Boxes = Vec<(u32, u32, u32, u32)>;

fn dragged(from: (u32, u32), to: (u32, u32)) -> (Boxes, Rect, Rect) {
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, layer(7, from.0, from.1, 960, 540, 2)).expect("room");
    let mut acc = DamageAccumulator::new();
    let rects = submit_layer(&mut scene, layer(7, to.0, to.1, 960, 540, 2)).expect("room");
    rects.into_iter().flatten().for_each(|r| acc.accumulate(r));
    let was = Rect { x: from.0, y: from.1, width: 960, height: 540 };
    let now = Rect { x: to.0, y: to.1, width: 960, height: 540 };
    (drained(&mut acc), was, now)
}

#[test]
fn a_drag_step_repaints_the_box_round_where_the_window_was_and_is() {
    let (got, was, now) = dragged((240, 135), (248, 139));
    assert_eq!(got, [tuple(was.union(now))]);
    let both = was.area() + now.area() - was.intersect(now).map_or(0, |r| r.area());
    assert_eq!(was.union(now).area() - both, 2 * 8 * 4, "two corners of 8 by 4");
}

#[test]
fn a_straight_drag_repaints_exactly_where_the_window_was_and_is() {
    let (got, was, now) = dragged((240, 135), (256, 135));
    assert_eq!(got, [tuple(was.union(now))]);
    let both = was.area() + now.area() - was.intersect(now).map_or(0, |r| r.area());
    assert_eq!(was.union(now).area(), both);
}

#[test]
fn a_window_thrown_clear_of_itself_repaints_its_two_places_only() {
    let (got, was, now) = dragged((0, 0), (960, 540));
    let mut want = vec![tuple(was), tuple(now)];
    want.sort_unstable();
    assert_eq!(got, want);
}

#[test]
fn a_resubmit_in_place_repaints_the_window_once() {
    let (got, was, _) = dragged((240, 135), (240, 135));
    assert_eq!(got, [tuple(was)]);
}

#[test]
fn a_long_diagonal_throw_repaints_the_two_places_not_their_box() {
    // The box would be 1360 by 840; the two apart, overlap twice, are less.
    let (got, was, now) = dragged((100, 100), (500, 400));
    assert!(was.touches(now) && !was.worth_one(now));
    let mut want = vec![tuple(was), tuple(now)];
    want.sort_unstable();
    assert_eq!(got, want);
}
