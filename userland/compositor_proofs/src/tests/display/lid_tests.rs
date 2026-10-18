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

//! Closed means nothing presented: one black frame goes out, and after it no
//! damage and no band change reaches the screen. Open redraws whole: the
//! screen equals one drawn whole for what the band shows now, the trusted
//! mark included, whatever changed while the lid was shut.

use super::lid_rig::{black, whole, Rig, Shows, H, W};
use crate::damage::Rect;
use crate::frame_pacer::indicator_band::band;
use crate::state::indicator::KIND_CAMERA;
use crate::state::lid_rule::Lid;

#[test]
fn closing_darkens_every_pixel_in_one_frame_then_presents_nothing() {
    let s = Shows { trusted: true, ..Shows::default() };
    let mut rig = Rig::new(s);
    rig.shell(true);
    rig.frame(s);
    assert_eq!((rig.lid, rig.presents), (Lid::Dark, 1));
    assert!(rig.screen == black());
    rig.acc.accumulate(Rect { x: 3, y: 4, width: 50, height: 20 });
    rig.frame(s);
    assert_eq!(rig.presents, 1, "dark: the damage waits, nothing goes out");
}

#[test]
fn opening_redraws_whole_with_the_band_as_it_is_now() {
    let mut s = Shows::default();
    let mut rig = Rig::new(s);
    rig.shell(true);
    rig.frame(s);
    // While shut: a camera comes on, the trusted band is held.
    s.ind.apply(KIND_CAMERA, true, 7).unwrap();
    s.trusted = true;
    rig.acc.accumulate(band(W, H));
    rig.frame(s);
    assert!(rig.screen == black());
    rig.shell(false);
    rig.frame(s);
    assert_eq!(rig.lid, Lid::Open);
    assert!(rig.screen == whole(Lid::Open, s), "the band and mark drawn whole");
}

#[test]
fn a_refused_black_frame_goes_again_before_the_lid_is_dark() {
    let s = Shows::default();
    let mut rig = Rig::new(s);
    rig.shell(true);
    rig.refuse = 1;
    rig.frame(s);
    assert_eq!((rig.lid, rig.presents), (Lid::Darkening, 0));
    rig.frame(s);
    assert_eq!((rig.lid, rig.presents), (Lid::Dark, 1));
    assert!(rig.screen == black());
}
