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

//! One owner, several windows: a Linux guest behind the Wayland bridge holds a
//! layer per toplevel and per popup in the application band. Each layer is
//! named by its surface handle, stacks and raises on its own, and goes alone.

use super::multi_rig::*;
use crate::damage::{DamageAccumulator, Rect};
use crate::scene::{Refused, SceneTable, MAX_LAYERS_PER_OWNER};
use crate::scene_raise::{raise_by_pid, raise_window};
use crate::scene_remove::{remove_by_pid, remove_layer};
use crate::scene_submit::{submit_keyed_layer, submit_layer};
use crate::tests::scene::frame_model::{layer, Rng, H, W};

const GUEST: u32 = 40;
const OTHER: u32 = 41;

fn order(scene: &SceneTable) -> Vec<u64> {
    let (layers, n) = scene.z_sorted_snapshot();
    layers[..n].iter().map(|l| l.surface_handle).collect()
}

fn three_windows() -> (SceneTable, DamageAccumulator) {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    put(&mut scene, &mut acc, keyed(GUEST, 1, 401, (2, 2, 30, 20)));
    put(&mut scene, &mut acc, keyed(OTHER, 7, 411, (10, 8, 30, 20)));
    put(&mut scene, &mut acc, keyed(GUEST, 2, 402, (20, 14, 30, 20)));
    (scene, acc)
}

#[test]
fn a_second_window_of_one_owner_in_one_band_is_a_layer_of_its_own() {
    let (scene, _) = three_windows();
    assert_eq!(scene.layers().count(), 3);
    assert_eq!(order(&scene), vec![401, 411, 402]);
}

#[test]
fn two_windows_of_one_owner_raise_independently() {
    let (mut scene, _) = three_windows();
    raise_window(&mut scene, GUEST, 1).expect("window 1 was covered");
    assert_eq!(order(&scene), vec![411, 402, 401], "only window 1 comes up");
    raise_window(&mut scene, OTHER, 7).expect("the other owner's window was covered");
    assert_eq!(order(&scene), vec![402, 401, 411]);
    raise_window(&mut scene, GUEST, 2).expect("window 2 was covered");
    assert_eq!(order(&scene), vec![401, 411, 402]);
    assert!(raise_window(&mut scene, GUEST, 2).is_none(), "on top already: nothing repaints");
}

#[test]
fn raising_an_owner_whole_keeps_its_windows_in_their_order() {
    let (mut scene, _) = three_windows();
    raise_window(&mut scene, GUEST, 1);
    // 411, 402, 401: the guest's whole raise lifts both over 411 and keeps
    // window 1 over window 2, as they were.
    raise_by_pid(&mut scene, GUEST);
    assert_eq!(order(&scene), vec![411, 402, 401]);
}

#[test]
fn a_resubmit_moves_only_the_layer_of_its_own_handle() {
    let (mut scene, mut acc) = three_windows();
    let before = scene.layers().find(|l| l.surface_handle == 401).copied().expect("401");
    put(&mut scene, &mut acc, keyed(GUEST, 2, 402, (30, 20, 30, 20)));
    assert_eq!(scene.layers().count(), 3);
    let after = scene.layers().find(|l| l.surface_handle == 401).copied().expect("401");
    assert_eq!((after.x, after.y, after.stack), (before.x, before.y, before.stack));
    assert_eq!(order(&scene), vec![401, 411, 402], "a move keeps the stack");
}

#[test]
fn a_replaced_handle_keeps_its_window_place_and_lets_the_old_surface_go() {
    let (mut scene, _) = three_windows();
    let wider = keyed(GUEST, 1, 405, (2, 2, 40, 26));
    let done = submit_keyed_layer(&mut scene, wider, Some(401)).expect("replace");
    assert_eq!(done.released, Some(401));
    assert_eq!(order(&scene), vec![405, 411, 402], "the resized window keeps its place");
    let rects: Vec<_> = done.repaint.iter().flatten().map(|r| (r.width, r.height)).collect();
    assert_eq!(rects, vec![(40, 26), (30, 20)], "new and old extent repaint");
}

#[test]
fn replacing_a_layer_the_owner_does_not_hold_is_refused() {
    let (mut scene, _) = three_windows();
    let l = keyed(GUEST, 1, 406, (2, 2, 30, 20));
    assert_eq!(submit_keyed_layer(&mut scene, l, Some(411)).err(), Some(Refused::NoLayer));
    assert_eq!(submit_keyed_layer(&mut scene, l, Some(999)).err(), Some(Refused::NoLayer));
    // Naming a handle that already has a layer as the new one would leave
    // two layers showing it.
    let twin = keyed(GUEST, 1, 402, (2, 2, 30, 20));
    assert_eq!(submit_keyed_layer(&mut scene, twin, Some(401)).err(), Some(Refused::NoLayer));
    assert_eq!(order(&scene), vec![401, 411, 402], "nothing changed");
}

#[test]
fn a_dialog_closing_leaves_the_window_under_it_byte_exact() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    put(&mut scene, &mut acc, keyed(GUEST, 1, 401, (4, 4, 48, 36)));
    put(&mut scene, &mut acc, keyed(OTHER, 7, 411, (40, 2, 20, 20)));
    let alone = Canvas::composed(&scene);
    put(&mut scene, &mut acc, keyed(GUEST, 3, 403, (14, 10, 24, 18)));
    let mut screen = Canvas::composed(&scene);
    while acc.drain().is_some() {}
    let gone = remove_layer(&mut scene, GUEST, APP_Z, 403).expect("the dialog's layer");
    assert_eq!(gone.released, Some(403));
    acc.accumulate(gone.rect);
    screen.repaint(&scene, &mut acc);
    for y in 0..H {
        for x in 0..W {
            assert_eq!(screen.at(x, y), alone.at(x, y), "pixel ({x}, {y})");
        }
    }
    assert!(scene.layers().any(|l| l.surface_handle == 401), "the main window stays");
}

#[test]
fn removing_one_layer_needs_its_owner_band_and_handle() {
    let (mut scene, _) = three_windows();
    assert!(remove_layer(&mut scene, OTHER, APP_Z, 401).is_none(), "not the sender's");
    assert!(remove_layer(&mut scene, GUEST, APP_Z + 1, 401).is_none(), "not in that band");
    assert!(remove_layer(&mut scene, GUEST, APP_Z, 401).is_some());
    assert_eq!(order(&scene), vec![411, 402]);
}

/// The window manager's rule, modelled per window: z is the order of raises,
/// and a click lands on the window with the highest z that contains it.
/// After any opens, raises and closes of several windows per owner, the
/// window drawn on top at every point is that window.
#[test]
fn the_window_drawn_on_top_is_the_one_a_click_there_lands_on() {
    for seed in 1..=48u64 {
        let mut rng = Rng::new(seed);
        let mut scene = SceneTable::new();
        // (owner, window, handle, rect, z)
        let mut wm: Vec<(u32, u32, u64, (u32, u32, u32, u32), u32)> = Vec::new();
        let mut z = 0u32;
        let mut next_handle = 1000u64;
        for step in 0..80 {
            let roll = rng.below(10);
            if wm.len() < 12 && (roll < 3 || wm.is_empty()) {
                let (owner, window) = (1 + rng.below(3), 1 + rng.below(6));
                if wm.iter().any(|w| w.0 == owner && w.1 == window) {
                    continue;
                }
                let (w, h) = (6 + rng.below(30), 6 + rng.below(24));
                let r = (rng.below(W - w), rng.below(H - h), w, h);
                next_handle += 1;
                let l = keyed(owner, window, next_handle, r);
                submit_keyed_layer(&mut scene, l, None).expect("room");
                z += 1;
                wm.push((owner, window, next_handle, r, z));
            } else if roll < 4 {
                let i = rng.below(wm.len() as u32) as usize;
                let (owner, _, handle, _, _) = wm.remove(i);
                remove_layer(&mut scene, owner, APP_Z, handle).expect("held");
            } else {
                let i = rng.below(wm.len() as u32) as usize;
                raise_window(&mut scene, wm[i].0, wm[i].1);
                z += 1;
                wm[i].4 = z;
            }
            let screen = Canvas::composed(&scene);
            for y in 0..H {
                for x in 0..W {
                    let hit = wm
                        .iter()
                        .filter(|w| {
                            let r = w.3;
                            x >= r.0 && x < r.0 + r.2 && y >= r.1 && y < r.1 + r.3
                        })
                        .max_by_key(|w| w.4)
                        .map_or(0, |w| w.2);
                    assert_eq!(screen.at(x, y).1, hit, "seed {seed} step {step} at ({x}, {y})");
                }
            }
        }
    }
}

#[test]
fn an_owner_holds_at_most_its_share_and_others_still_fit() {
    let mut scene = SceneTable::new();
    for i in 0..MAX_LAYERS_PER_OWNER as u64 {
        let l = keyed(GUEST, 1 + i as u32, 500 + i, (0, 0, 4, 4));
        submit_keyed_layer(&mut scene, l, None).expect("under the share");
    }
    let one_more = keyed(GUEST, 99, 999, (0, 0, 4, 4));
    assert_eq!(submit_keyed_layer(&mut scene, one_more, None).err(), Some(Refused::Full));
    assert!(submit_layer(&mut scene, layer(GUEST, 0, 0, 4, 4, 9)).is_err(), "nor the old way");
    // Its held layers still move and take new surfaces, and others fit.
    let moved = keyed(GUEST, 1, 500, (2, 2, 4, 4));
    submit_keyed_layer(&mut scene, moved, None).expect("a move of a held layer");
    let resized = keyed(GUEST, 1, 600, (2, 2, 8, 8));
    submit_keyed_layer(&mut scene, resized, Some(500)).expect("a replace of a held layer");
    submit_keyed_layer(&mut scene, keyed(OTHER, 1, 700, (0, 0, 4, 4)), None).expect("another");
}

#[test]
fn the_table_refuses_past_its_size() {
    let mut scene = SceneTable::new();
    let mut taken = 0u32;
    'fill: for owner in 1..=64u32 {
        for w in 0..MAX_LAYERS_PER_OWNER as u32 {
            let l = keyed(owner, w + 1, (owner as u64) << 8 | w as u64, (0, 0, 4, 4));
            if submit_keyed_layer(&mut scene, l, None).is_err() {
                break 'fill;
            }
            taken += 1;
        }
    }
    assert_eq!(taken, 64, "scene/layer.rs MAX_LAYERS");
    assert_eq!(scene.layers().count(), 64);
}

#[test]
fn an_old_client_is_served_as_before() {
    // A client that submits the old way: one layer per band, a resubmit with
    // a new handle replaces it, and a raise or a remove covers all it holds.
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, layer(OTHER, 0, 0, 10, 10, APP_Z)).expect("old way");
    submit_layer(&mut scene, keyed_old(OTHER, 77, (5, 5, 10, 10))).expect("new surface");
    assert_eq!(scene.layers().count(), 1);
    assert_eq!(scene.layers().next().map(|l| l.surface_handle), Some(77));
    // A keyed client beside it is not touched by its submits.
    submit_keyed_layer(&mut scene, keyed(GUEST, 1, 401, (0, 0, 8, 8)), None).expect("keyed");
    submit_layer(&mut scene, keyed_old(OTHER, 78, (5, 5, 10, 10))).expect("again");
    assert_eq!(order(&scene), vec![78, 401]);
    // The window manager raising one of its windows by id raises its layer.
    raise_window(&mut scene, OTHER, 3).expect("covered");
    assert_eq!(order(&scene), vec![401, 78]);
    assert!(remove_by_pid(&mut scene, OTHER).is_some());
    assert_eq!(order(&scene), vec![401]);
}

#[test]
fn an_old_submit_never_replaces_a_keyed_layer_of_the_same_owner() {
    let mut scene = SceneTable::new();
    submit_keyed_layer(&mut scene, keyed(GUEST, 1, 401, (0, 0, 8, 8)), None).expect("keyed");
    submit_layer(&mut scene, keyed_old(GUEST, 90, (0, 0, 8, 8))).expect("old way");
    assert_eq!(scene.layers().count(), 2);
    let tagged = scene.layers().find(|l| l.surface_handle == 401).expect("kept");
    assert_eq!(tagged.window_id, 1);
}

#[test]
fn a_whole_owner_remove_takes_its_keyed_layers_too() {
    let (mut scene, _) = three_windows();
    let r = remove_by_pid(&mut scene, GUEST).expect("held two");
    assert_eq!(order(&scene), vec![411]);
    assert_eq!((r.x, r.y), (2, 2));
    let _: Rect = r;
}

fn keyed_old(owner: u32, handle: u64, r: (u32, u32, u32, u32)) -> crate::scene::Layer {
    crate::scene::Layer { surface_handle: handle, ..layer(owner, r.0, r.1, r.2, r.3, APP_Z) }
}

#[test]
fn a_present_swaps_the_keyed_layer_it_names_and_never_adds_one() {
    use crate::state::present_rule::present;
    let changed = Rect { x: 0, y: 0, width: 4, height: 4 };
    let mut scene = SceneTable::new();
    submit_keyed_layer(&mut scene, keyed(GUEST, 1, 401, (0, 0, 8, 8)), None).expect("keyed");
    // Its one keyed layer in the band takes the next buffer.
    let s = present(&mut scene, GUEST, APP_Z, 501, changed).expect("one keyed layer");
    assert_eq!(s.old, Some(401));
    assert_eq!(order(&scene), vec![501]);
    // With two, a present names a layer by a handle it shows, or none.
    submit_keyed_layer(&mut scene, keyed(GUEST, 2, 402, (9, 9, 8, 8)), None).expect("keyed");
    assert!(present(&mut scene, GUEST, APP_Z, 502, changed).is_none(), "no guess");
    let s = present(&mut scene, GUEST, APP_Z, 402, changed).expect("named by its handle");
    assert_eq!(s.old, None);
    assert_eq!(order(&scene), vec![501, 402]);
    let tagged = scene.layers().find(|l| l.surface_handle == 501).expect("swapped");
    assert_eq!((tagged.window_id, tagged.keyed), (1, true));
}
