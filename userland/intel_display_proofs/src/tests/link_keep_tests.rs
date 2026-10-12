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

//! A DisplayPort output whose link gives way is trained again at the mode
//! it shows, on the surfaces it shows, and STATE says LOST once; a link
//! that holds is left alone.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use super::dp_rig::dp_rig;
use super::edid_blob::edid_1080p_and_xga;
use super::two_rig::TOTAL;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::timing;
use crate::model::firmware::T1080;

#[test]
fn a_link_that_gives_way_is_trained_again_at_the_mode_shown() {
    let (mut d, model, sink) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    // The model's firmware lit the transcoder without training its sink:
    // the first look finds no link and trains one.
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT | LOST);
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT, "the link now holds");
    sink.borrow_mut().lock_mbps = 2700;
    let s = d.state(0).unwrap();
    assert_eq!(s.flags, SCANNING_OUT | LOST, "the cable gave way, and was trained again");
    assert_eq!(sink.borrow().link.map(|l| l.0), Some(2700), "at the rate it still takes");
    let mut m = model.borrow_mut();
    let t = timing::read(&mut *m, Transcoder::B).unwrap();
    assert_eq!((t.h_active, t.v_active), (1920, 1080), "the same mode");
    assert_eq!(m.pixel(Pipe::B, 9, 9), Some(0xFF00_00AA), "the same picture");
}

#[test]
fn a_link_that_cannot_be_brought_back_is_lost_not_scanning_out() {
    let (mut d, _, sink) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    let _ = d.state(0);
    sink.borrow_mut().lock_lanes = 0;
    assert_eq!(d.state(0).unwrap().flags & (LOST | SCANNING_OUT), LOST);
}
