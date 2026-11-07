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

use crate::state::direct::{Direct, REMEMBERED};

#[test]
fn only_the_driver_caps_named_may_be_asked() {
    let d = Direct::new(40, true);
    assert!(d.able(40));
    assert!(!d.able(41), "a restarted driver is primed again first");
    assert!(!Direct::new(40, false).able(40), "no DIRECT_SCANOUT or no BAND_PLANE");
    let mut off = d;
    off.refuse_driver();
    assert!(!off.able(40), "ENODEV or ENOSYS: never asked again");
}

#[test]
fn a_refused_buffer_is_remembered_until_newer_ones_push_it_out() {
    let mut d = Direct::new(1, true);
    d.refuse(100);
    d.refuse(100);
    assert!(d.refused(100) && !d.refused(101) && !d.refused(0));
    for h in 0..REMEMBERED as u64 {
        d.refuse(200 + h);
    }
    assert!(!d.refused(100), "the oldest goes first");
    assert!(d.refused(200 + REMEMBERED as u64 - 1));
}

#[test]
fn opacity_follows_the_last_present_of_each_buffer() {
    let mut d = Direct::new(1, true);
    assert!(!d.opaque(5), "never presented");
    d.presented(5, true);
    d.presented(6, true);
    assert!(d.opaque(5) && d.opaque(6));
    d.presented(5, false);
    assert!(!d.opaque(5) && d.opaque(6), "a translucent present takes it back");
    assert!(!d.opaque(0));
}
