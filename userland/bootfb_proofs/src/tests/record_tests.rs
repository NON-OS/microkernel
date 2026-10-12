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

//! Records bootfb leaves alone, and modes the mapped range cannot hold:
//! nothing asked of the kernel for the first, the claim released for the
//! second.

use super::firmware_model::Model;
use crate::bring_up::bring_up;
use crate::firmware::Why;
use crate::geometry::BGRX;
use crate::record::Listed;

#[test]
fn any_other_record_is_left_alone() {
    let (mut m, base) = Model::new(64, 16, 256, BGRX, 0x40);
    let bad: [fn(&mut Listed); 7] = [
        |r| r.bus_kind = 4,
        |r| r.device = 2,
        |r| r.vendor = 0x8086,
        |r| r.bar_count = 0,
        |r| r.bar0.1 = 0,
        |r| r.bar0.2 += 0x40,
        |r| r.bar0.4 = 4096,
    ];
    for f in bad {
        let mut r = base;
        f(&mut r);
        assert_eq!(bring_up(&mut m, &r).err(), Some(Why::Record));
    }
    assert!(m.calls.is_empty(), "not even announced");
}

#[test]
fn a_mode_the_range_does_not_hold_releases_the_claim() {
    let (mut m, mut r) = Model::new(64, 16, 256, BGRX, 0x40);
    r.bar0.4 = 0x80;
    assert_eq!(bring_up(&mut m, &r).err(), Some(Why::Shape), "aux and the mode disagree");
    assert_eq!(m.calls.last(), Some(&"release"));
    let (mut m, mut r) = Model::new(64, 16, 256, BGRX, 0);
    r.bar0.3 -= 4;
    assert_eq!(bring_up(&mut m, &r).err(), Some(Why::Shape), "one pixel short");
    assert_eq!(m.calls, ["announce", "claim", "geometry", "map", "release"]);
}
