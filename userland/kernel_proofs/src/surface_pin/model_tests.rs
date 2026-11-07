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

//! The model's pins, unpins, unmaps and exits, in set orders and at random.

use super::model::{World, PAGES};

fn mapped(g: u32, n: usize) -> World {
    let mut w = World::new();
    (0..n).for_each(|p| w.map(g, p));
    w
}

#[test]
fn a_pinned_window_is_read_in_place_read_only() {
    let mut w = mapped(1, 4);
    let h = w.pin(1, 0, 4).expect("pinned");
    w.attach(h);
    assert_eq!(w.views.len(), 4);
    assert!(w.views.iter().all(|v| !v.write));
    w.check();
}

#[test]
fn an_unpinned_surface_is_never_attached_again() {
    let mut w = mapped(1, 2);
    let h = w.pin(1, 0, 2).unwrap();
    w.unpin(h);
    w.attach(h);
    assert!(w.views.is_empty());
    let viewed = w.pin(1, 0, 2).unwrap();
    w.attach(viewed);
    w.unpin(viewed);
    w.attach(viewed);
    assert_eq!(w.views.len(), 2, "the view mapped before the unpin, and no other");
    assert_eq!(w.refused_after_revoke, 1);
    w.check();
}

#[test]
fn a_frame_is_pinned_once_and_a_page_the_guest_lacks_or_may_run_not_at_all() {
    let mut w = mapped(1, 4);
    let h = w.pin(1, 0, 2).unwrap();
    assert!(w.pin(1, 1, 2).is_none(), "page 1 is in a surface already");
    assert!(w.pin(1, 2, 4).is_none(), "pages 4 and 5 are not mapped");
    w.unpin(h);
    w.protect_exec(1, 3);
    assert!(w.pin(1, 2, 2).is_none(), "page 3 may run");
    assert!(w.pin(1, 2, 1).is_some());
}

#[test]
fn reprotecting_a_pinned_page_unpins_its_surface() {
    let mut w = mapped(1, 2);
    let h = w.pin(1, 0, 2).unwrap();
    w.protect_exec(1, 1);
    w.attach(h);
    assert!(w.views.is_empty());
    assert!(w.surfaces.is_empty(), "unviewed, the slot went with the unpin");
}

#[test]
fn an_unmap_under_a_view_frees_nothing_the_view_reads_until_it_lets_go() {
    let mut w = mapped(1, 4);
    let h = w.pin(1, 0, 4).unwrap();
    w.attach(h);
    w.unmap(1, 0, 4);
    w.check();
    (0..PAGES).for_each(|p| w.map(2, p));
    w.check();
    w.let_go(h);
    let view_frames: Vec<u64> = (4..8).map(|f| f * 4096).collect();
    assert!(view_frames.iter().all(|f| w.frames[f] != super::model::Owner::Registry));
    w.check();
}

#[test]
fn a_pin_unmapped_in_parts_under_a_view_is_freed_whole_once_the_view_lets_go() {
    // The supervisor unmaps a span a MiB at a time: a buffer over a MiB
    // comes down in parts, here two of two pages each.
    let mut w = mapped(1, 4);
    let h = w.pin(1, 0, 4).unwrap();
    w.attach(h);
    w.unmap(1, 0, 2);
    w.check();
    w.unmap(1, 2, 4);
    w.check();
    w.let_go(h);
    w.check();
    assert_eq!(w.held(), 0, "every frame of the pinned buffer back");
}

#[test]
fn a_view_let_go_between_the_parts_of_an_unmap_frees_each_part() {
    let mut w = mapped(1, 4);
    let h = w.pin(1, 0, 4).unwrap();
    w.attach(h);
    w.unmap(1, 0, 2);
    w.let_go(h);
    w.check();
    w.unmap(1, 2, 4);
    w.check();
    assert_eq!(w.held(), 0, "the first part freed with the view, the rest by its unmap");
}

#[test]
fn an_exit_after_part_of_a_viewed_pin_was_unmapped_keeps_the_rest_for_the_view() {
    let mut w = mapped(1, 4);
    let h = w.pin(1, 0, 4).unwrap();
    w.attach(h);
    w.unmap(1, 0, 1);
    w.exit(1);
    (0..PAGES).for_each(|p| w.map(2, p));
    w.check();
    w.let_go(h);
    w.check();
    (0..PAGES).for_each(|p| w.unmap(2, p, p + 1));
    assert_eq!(w.held(), 0);
}

#[test]
fn an_exit_under_a_view_leaves_its_frames_to_the_registry() {
    let mut w = mapped(1, 3);
    let h = w.pin(1, 0, 3).unwrap();
    w.attach(h);
    w.exit(1);
    (0..PAGES).for_each(|p| w.map(2, p));
    w.check();
    w.attach(h);
    assert_eq!(w.views.len(), 3, "no attach after the exit");
    w.let_go(h);
    w.check();
}

#[test]
fn a_supervisor_s_guests_hold_sixteen_surfaces_and_no_more() {
    let mut w = World::new();
    let mut made = 0;
    for g in 1..=3 {
        (0..PAGES).for_each(|p| w.map(g, p));
    }
    for _ in 0..3 {
        for p in 0..PAGES {
            if w.pin(1, p, 1).is_some() {
                made += 1;
            }
        }
        let first: Vec<u64> = w.surfaces.iter().map(|s| s.handle).take(4).collect();
        first.into_iter().for_each(|h| w.attach(h));
        w.unmap(1, 0, PAGES);
        (0..PAGES).for_each(|p| w.map(1, p));
    }
    assert!(w.surfaces.iter().filter(|s| s.owner == 1).count() <= 16);
    assert!(made <= 24);
    w.check();
}

#[test]
#[should_panic(expected = "reads freed frame")]
fn freeing_a_viewed_frame_is_caught() {
    let mut w = mapped(1, 1);
    let h = w.pin(1, 0, 1).unwrap();
    w.attach(h);
    let f = w.views[0].frame;
    w.free(f);
    w.check();
}

#[test]
fn in_any_order_no_view_reads_a_freed_or_reused_frame() {
    let mut seed = 0x2545_F491_4F6C_DD1Du64;
    let mut next = |n: u64| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed % n
    };
    for _ in 0..3000 {
        let mut w = World::new();
        let mut handles = Vec::new();
        for _ in 0..40 {
            let g = 1 + next(3) as u32;
            let p = next(PAGES as u64) as usize;
            let n = 1 + next(3) as usize;
            match next(8) {
                0 | 1 => (p..(p + n).min(PAGES)).for_each(|q| w.map(g, q)),
                2 => handles.extend(w.pin(g, p, n.min(PAGES - p))),
                3 => {
                    if let Some(&h) = handles.get(next(handles.len().max(1) as u64) as usize) {
                        w.attach(h);
                    }
                }
                4 => {
                    if let Some(&h) = handles.get(next(handles.len().max(1) as u64) as usize) {
                        if next(2) == 0 {
                            w.unpin(h)
                        } else {
                            w.let_go(h)
                        }
                    }
                }
                5 => w.unmap(g, p, (p + n).min(PAGES)),
                6 => w.protect_exec(g, p),
                _ => {
                    if next(4) == 0 {
                        w.exit(g)
                    }
                }
            }
            w.check();
        }
        // Every view let go and every guest gone: nothing is left held.
        handles.iter().for_each(|&h| w.let_go(h));
        (1..=3).for_each(|g| w.exit(g));
        w.check();
        assert_eq!(w.held(), 0, "a frame was never given back");
    }
}
