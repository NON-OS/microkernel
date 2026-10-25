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

//! A brightness key through the compositor: the driver's answer read only
//! as far as it is whole, the focused output's panel picked (else the first
//! listed), and the level moved to the next one listed, never past an end.

use crate::display::brightness_list::{panels, set_request, MAX_PANELS};
use crate::display::brightness_rule::{identity, next, Panel};

/// A BRIGHTNESS answer, as F 12 lays it out: status, count, then each
/// panel's identity, level now, level count and levels.
pub(super) fn answer(status: i32, list: &[(u32, u8, Vec<u8>)]) -> Vec<u8> {
    let mut a = status.to_le_bytes().to_vec();
    a.push(list.len() as u8);
    for (id, now, levels) in list {
        a.extend(id.to_le_bytes());
        a.extend([*now, levels.len() as u8]);
        a.extend(levels);
    }
    a
}

pub(super) fn read(a: &[u8]) -> Vec<Panel<'_>> {
    let mut out = [None; MAX_PANELS];
    let n = panels(a, &mut out);
    out.iter().take(n).flatten().copied().collect()
}

#[test]
fn the_answer_is_read_only_as_far_as_it_is_whole() {
    let all: Vec<u8> = (1..=100).collect();
    let a = answer(0, &[(identity(0), 60, all.clone()), (identity(1), 30, vec![5, 40, 100])]);
    let p = read(&a);
    assert_eq!(p.len(), 2);
    assert_eq!((p[1].identity, p[1].now, p[1].levels), (0x8000_0001, 30, &[5, 40, 100][..]));
    assert_eq!(read(&a[..a.len() - 1]).len(), 1, "a panel cut short is dropped");
    assert!(read(&answer(-5, &[(identity(0), 60, all)])).is_empty(), "a refusal lists none");
    assert!(read(&[0, 0, 0]).is_empty(), "no status");
    let many: Vec<_> = (0..20).map(|o| (identity(o), 50, vec![50])).collect();
    assert_eq!(read(&answer(0, &many)).len(), MAX_PANELS, "kept to the bound");
}

#[test]
fn a_key_moves_to_the_next_listed_level_and_stops_at_each_end() {
    let levels = [5, 40, 100];
    let at = |now: u8| Panel { identity: 0, now, levels: &levels };
    assert_eq!((next(&at(40), true), next(&at(40), false)), (Some(100), Some(5)));
    assert_eq!((next(&at(100), true), next(&at(5), false)), (None, None), "the ends hold");
    assert_eq!((next(&at(50), true), next(&at(50), false)), (Some(100), Some(40)));
    assert_eq!(next(&at(0xFF), true), None, "unknown is not stepped from");
    let all: Vec<u8> = (1..=100).collect();
    let p = Panel { identity: 0, now: 60, levels: &all };
    assert_eq!((next(&p, true), next(&p, false)), (Some(61), Some(59)));
    assert_eq!(set_request(61, identity(1)), [4, 61, 1, 0, 0, 0x80]);
}
