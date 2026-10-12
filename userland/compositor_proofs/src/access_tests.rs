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

//! ACCESSIBILITY's body: every setting in its range is taken for its
//! output, and anything else (a fifth output, an unknown filter, a step the
//! magnifier does not have, a short body) is refused.

use crate::display::access_rule::{parse, Access, FILTER_TRITAN, FOLLOW_FOCUS, MAG_STEPS};

fn body(v: [u32; 5]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

#[test]
fn settings_in_range_are_taken() {
    let (out, a) = parse(&body([2, FILTER_TRITAN, 1, 8, FOLLOW_FOCUS]), 4).unwrap();
    assert_eq!(out, 2);
    assert_eq!(a, Access { filter: FILTER_TRITAN, contrast: true, magnify: 8, follow: 1 });
    assert!(a.magnified());
    for m in MAG_STEPS {
        assert_eq!(parse(&body([0, 0, 0, m, 0]), 4).unwrap().1.magnified(), m > 4);
    }
    assert!(!Access::default().magnified());
}

#[test]
fn anything_else_is_refused() {
    for bad in [[4, 0, 0, 4, 0], [0, 5, 0, 4, 0], [0, 0, 2, 4, 0], [0, 0, 0, 5, 0], [0, 0, 0, 4, 2]]
    {
        assert_eq!(parse(&body(bad), 4), None, "{bad:?}");
    }
    assert_eq!(parse(&body([0, 0, 0, 4, 0])[..16], 4), None, "short");
}
