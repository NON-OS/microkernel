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

//! A munmap of part of a map leaves what lies outside the hole, each part
//! still showing the bytes of the file it showed.

use super::super::random::Regs;
use super::support::{map, writer, P};
use crate::linux::file::shared_maps::rules::cut;
use crate::linux::file::shared_maps::spans::outside;

#[test]
fn a_hole_in_the_middle_splits_the_map_in_two() {
    let e = writer(map(1, 16, 6, "/f", 2));
    let [below, above] = outside(&e, 18 * P, 2 * P);
    assert_eq!(below, Some(writer(map(1, 16, 2, "/f", 2))));
    assert_eq!(above, Some(writer(map(1, 20, 2, "/f", 6))), "the file offset moves with it");
}

#[test]
fn a_hole_at_either_end_leaves_one_part_and_over_all_of_it_none() {
    let e = map(1, 16, 4, "/f", 0);
    assert_eq!(outside(&e, 14 * P, 3 * P), [None, Some(map(1, 17, 3, "/f", 1))]);
    assert_eq!(outside(&e, 19 * P, 9 * P), [Some(map(1, 16, 3, "/f", 0)), None]);
    assert_eq!(outside(&e, 16 * P, 4 * P), [None, None]);
    assert_eq!(outside(&e, 0, u64::MAX), [None, None], "a span that wraps takes all");
    assert_eq!(outside(&e, 20 * P, P), [Some(e.clone()), None], "the page after");
}

#[test]
fn a_munmap_cuts_only_its_own_process_maps() {
    let all = vec![map(1, 16, 4, "/f", 0), map(2, 16, 4, "/f", 0)];
    let want = vec![map(1, 16, 1, "/f", 0), map(1, 18, 2, "/f", 2), map(2, 16, 4, "/f", 0)];
    assert_eq!(cut(&all, 1, 17 * P, P), want, "process 2's map is not its");
}

/// Over random maps and holes: the parts lie inside the map and outside
/// the hole, they add up to what the hole missed, and each part shows the
/// same file byte at each address the map did.
#[test]
fn the_parts_keep_every_address_on_its_file_byte() {
    let mut r = Regs::new(0x5917_7A11);
    for _ in 0..100_000 {
        let e = map(1, r.small(64), 1 + r.small(16), "/f", r.small(64));
        let (at, span) = (r.small(96) * P, r.small(32) * P);
        let parts = outside(&e, at, span);
        let mut kept = 0;
        for p in parts.iter().flatten() {
            let (lo, hi) = p.place();
            assert!(e.at <= lo && hi <= e.at + e.span && p.span > 0);
            assert!(span == 0 || hi <= at || lo >= at + span, "a part inside the hole");
            assert_eq!(p.off - e.off, p.at - e.at, "a part moved off its file bytes");
            kept += p.span;
        }
        let gone = (e.at + e.span).min(at + span).saturating_sub(e.at.max(at));
        assert_eq!(kept + gone, e.span);
    }
}
