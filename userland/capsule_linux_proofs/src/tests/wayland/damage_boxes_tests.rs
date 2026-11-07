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

//! The boxes a client said changed, kept apart (wayland/damage_boxes.rs):
//! every pixel said is in a box kept, every box kept lies inside the one
//! box round all said (damage.rs), there are never more than four, rows
//! said one by one are one box and two rows far apart stay two; and moved
//! and cut to the box a commit shows, each lies inside it and still holds
//! every pixel said there.

use std::collections::BTreeSet;

use crate::damage::Damage;
use crate::damage_boxes::{Boxes, Corners, MAX_BOXES};

type Named = (i32, i32, i32, i32);

fn pixels((x0, y0, x1, y1): Corners) -> impl Iterator<Item = (u32, u32)> {
    (y0..y1).flat_map(move |y| (x0..x1).map(move |x| (x, y)))
}

fn kept(b: &Boxes) -> BTreeSet<(u32, u32)> {
    b.iter().flat_map(pixels).collect()
}

fn add_all(named: &[Named], scale: u32) -> (Boxes, Damage) {
    named
        .iter()
        .fold((Boxes::default(), Damage::None), |(b, d), &n| (b.add(n, scale), d.add(n, scale)))
}

/// A small generator, so a failing run names its seed.
struct Rng(u64);

impl Rng {
    fn next(&mut self, n: i32) -> i32 {
        self.0 =
            self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 33) % n as u64) as i32
    }
}

fn named(r: &mut Rng) -> Vec<Named> {
    (0..1 + r.next(9))
        .map(|_| (r.next(70) - 6, r.next(50) - 6, r.next(30) - 2, r.next(20) - 2))
        .collect()
}

#[test]
fn every_pixel_said_is_kept_and_every_box_lies_inside_the_one_round_them_all() {
    for seed in 0..3000u64 {
        let mut r = Rng(seed);
        let n = named(&mut r);
        let scale = 1 + r.next(2) as u32;
        let (b, d) = add_all(&n, scale);
        assert!(b.count() <= MAX_BOXES, "seed {seed}");
        let said: BTreeSet<(u32, u32)> = n
            .iter()
            .flat_map(|&n| Boxes::default().add(n, scale).iter().collect::<Vec<_>>())
            .flat_map(pixels)
            .collect();
        let kept = kept(&b);
        assert!(said.is_subset(&kept), "seed {seed}: a pixel said is in no box kept");
        match d {
            Damage::None => assert_eq!(b.count(), 0, "seed {seed}"),
            Damage::Rect(x0, y0, x1, y1) => {
                for (bx0, by0, bx1, by1) in b.iter() {
                    assert!(bx0 >= x0 && by0 >= y0 && bx1 <= x1 && by1 <= y1, "seed {seed}");
                    assert!(bx0 < bx1 && by0 < by1, "seed {seed}: an empty box kept");
                }
            }
            Damage::Whole => unreachable!("add never makes Whole"),
        }
    }
}

#[test]
fn moved_and_cut_to_the_box_shown_each_lies_inside_it_and_holds_what_was_said_there() {
    for seed in 0..2000u64 {
        let mut r = Rng(seed);
        let (b, _) = add_all(&named(&mut r), 1);
        let cut = (r.next(8) as u32, r.next(8) as u32);
        let (x0, y0) = (r.next(40) as u32, r.next(30) as u32);
        let bound = (x0, y0, x0 + 1 + r.next(40) as u32, y0 + 1 + r.next(30) as u32);
        let w = b.within(cut, bound);
        let inside =
            |&(x, y): &(u32, u32)| x >= bound.0 && y >= bound.1 && x < bound.2 && y < bound.3;
        let moved: BTreeSet<(u32, u32)> = kept(&b)
            .into_iter()
            .filter(|&(x, y)| x >= cut.0 && y >= cut.1)
            .map(|(x, y)| (x - cut.0, y - cut.1))
            .filter(inside)
            .collect();
        let got = kept(&w);
        assert!(got.iter().all(inside), "seed {seed}: a box past the box shown");
        assert_eq!(got, moved, "seed {seed}");
        assert_eq!(b.apart(cut, bound).is_some(), w.count() >= 2, "seed {seed}");
        let (areas, n) = w.areas();
        let back: Vec<Corners> =
            areas[..n].iter().map(|&(x, y, aw, ah)| (x, y, x + aw, y + ah)).collect();
        assert_eq!(back, w.iter().collect::<Vec<_>>(), "seed {seed}");
    }
}

#[test]
fn rows_said_one_by_one_are_one_box() {
    let rows: Vec<Named> = (0..30).map(|y| (0, 100 + y, 1300, 1)).collect();
    let (b, _) = add_all(&rows, 1);
    assert_eq!(b.iter().collect::<Vec<_>>(), [(0, 100, 1300, 130)]);
}

#[test]
fn a_selection_near_the_top_and_the_last_row_stay_two_boxes() {
    let (b, d) = add_all(&[(0, 40, 1300, 34), (0, 716, 1300, 34)], 1);
    assert_eq!(d, Damage::Rect(0, 40, 1300, 750), "the one box holds all 710 rows");
    assert_eq!(b.iter().collect::<Vec<_>>(), [(0, 40, 1300, 74), (0, 716, 1300, 750)]);
    let apart = b.apart((0, 0), (0, 40, 1300, 750)).expect("two");
    assert_eq!(apart.areas().1, 2);
}

#[test]
fn a_fifth_box_apart_joins_the_one_it_grows_least() {
    let four: [Named; 4] = [(0, 0, 10, 10), (500, 0, 10, 10), (0, 500, 10, 10), (500, 500, 10, 10)];
    let (b, _) = add_all(&four, 1);
    assert_eq!(b.count(), 4);
    let b = b.add((20, 0, 10, 10), 1);
    assert_eq!(b.count(), 4);
    assert!(b.iter().any(|c| c == (0, 0, 30, 10)), "{:?}", b.iter().collect::<Vec<_>>());
}

#[test]
fn boxes_are_taken_in_buffer_pixels_as_the_one_box_is() {
    for n in [
        (3, 4, 1, 1),
        (-5, -5, 20, 20),
        (0, 0, i32::MAX, i32::MAX),
        (1, 1, 0, 4),
        (i32::MAX, 0, 5, 5),
    ] {
        for scale in [1, 2, 3] {
            let (b, d) = add_all(&[n], scale);
            let one = b
                .iter()
                .next()
                .map_or(Damage::None, |(x0, y0, x1, y1)| Damage::Rect(x0, y0, x1, y1));
            assert_eq!(one, d, "{n:?} at {scale}");
        }
    }
}

/// One of a few words a guest may send: the ends of i32 and u32, zero,
/// small numbers, or any word.
fn edge(r: &mut Rng) -> i32 {
    const EDGES: [i32; 8] = [0, 1, -1, 64, i32::MAX, i32::MIN, i32::MAX - 1, 1 << 30];
    match r.next(3) {
        0 => EDGES[r.next(EDGES.len() as i32) as usize],
        1 => r.next(2000) - 100,
        _ => (r.next(1 << 16) << 16) ^ r.next(1 << 16),
    }
}

#[test]
fn any_words_named_at_any_scale_keep_the_rules() {
    use crate::damage_boxes_check::check;
    for seed in 0..4000u64 {
        let mut r = Rng(seed);
        let named: Vec<Named> = (0..1 + r.next(12))
            .map(|_| (edge(&mut r), edge(&mut r), edge(&mut r), edge(&mut r)))
            .collect();
        let cut = (edge(&mut r) as u32, edge(&mut r) as u32);
        let bound =
            (edge(&mut r) as u32, edge(&mut r) as u32, edge(&mut r) as u32, edge(&mut r) as u32);
        check(&named, r.next(5) as u32, cut, bound);
    }
}

#[test]
fn any_bytes_keep_the_rules() {
    use crate::damage_boxes_check::check_bytes;
    check_bytes(&[]);
    check_bytes(&[0xff; 27]);
    check_bytes(&[0xff; 28 + 16 * 17 + 3]);
    let mut bytes = vec![0u8; 28];
    bytes[0] = 1;
    for (x, y) in [(0i32, 0i32), (500, 0), (0, 500), (500, 500), (20, 0)] {
        for v in [x, y, 10, 10] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
    }
    bytes[20..24].copy_from_slice(&u32::MAX.to_le_bytes());
    bytes[24..28].copy_from_slice(&u32::MAX.to_le_bytes());
    check_bytes(&bytes);
}
