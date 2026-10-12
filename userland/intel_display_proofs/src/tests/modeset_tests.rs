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

//! A DisplayPort mode set end to end, and its refusal when the trained
//! link cannot carry the mode.

use super::port_tests::board;
use crate::dp::caps::Caps;
use crate::engine::aux_regs::Ddi;
use crate::engine::ddi::buf_ctl;
use crate::engine::error::Error;
use crate::engine::mn::{compute, TU_64};
use crate::engine::modeset::enable_dp;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::port::Port;
use crate::engine::regs::{trans_base, trans_conf, trans_ddi_func_ctl};
use crate::engine::timing;
use crate::model::Ticks;

#[test]
fn ten_eighty_p_on_ddi_b_lights_transcoder_b() {
    let (mut m, _) = board(Ddi::B);
    let clock = Ticks::new();
    let t = nonos_modes::dmt(1920, 1080, 60, false).unwrap().timing;
    let mut p = Port { mmio: &mut m, ddi: Ddi::B, dpll: 1, enhanced: true, clock: &clock };
    let caps = Caps::read(&mut p).unwrap().unwrap();
    let link = enable_dp(&mut p, Transcoder::B, Pipe::B, &t, &caps, (5400, 4)).unwrap();
    assert_eq!((link.mbps, link.lanes), (5400, 4));
    assert_eq!(timing::read(&mut m, Transcoder::B).unwrap().h_total(), 2200);
    let mn = compute(148_500, 24, 4, 5400).unwrap();
    assert_eq!(m.get(trans_base(Transcoder::B) + 0x30), TU_64 | mn.data_m);
    assert_eq!(m.get(trans_ddi_func_ctl(Transcoder::B)) >> 24 & 7, 2, "DP SST");
    assert_eq!(m.get(trans_conf(Transcoder::B)) & (1 << 30), 1 << 30);
}

#[test]
fn a_link_too_narrow_for_the_mode_is_taken_down() {
    let (mut m, s) = board(Ddi::B);
    s.borrow_mut().lock_mbps = 1620;
    s.borrow_mut().lock_lanes = 1;
    let clock = Ticks::new();
    let t = nonos_modes::dmt(1920, 1080, 60, false).unwrap().timing;
    let mut p = Port { mmio: &mut m, ddi: Ddi::B, dpll: 1, enhanced: true, clock: &clock };
    let caps = Caps::read(&mut p).unwrap().unwrap();
    let r = enable_dp(&mut p, Transcoder::B, Pipe::B, &t, &caps, (5400, 4));
    assert_eq!(r.err(), Some(Error::Argument));
    assert_eq!(m.get(buf_ctl(1)) & (1 << 31), 0, "port down");
    assert_eq!(m.get(trans_conf(Transcoder::B)), 0, "transcoder never enabled");
}
