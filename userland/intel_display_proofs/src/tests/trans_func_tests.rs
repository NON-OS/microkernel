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

//! TRANS_DDI_FUNC_CTL words and the transcoder's enable.

use crate::engine::error::Error;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::regs::{trans_conf, trans_ddi_func_ctl};
use crate::engine::trans_func::{disable, enable, func_ctl, Protocol};
use crate::model::{Gen9, Polls};
use nonos_modes::Detailed;

fn t(hp: bool, vp: bool) -> Detailed {
    let mut d = nonos_modes::dmt(1920, 1080, 60, false).unwrap().timing;
    d.h_positive = hp;
    d.v_positive = vp;
    d
}

#[test]
fn each_field_lands_where_the_prm_puts_it() {
    let dp = func_ctl(Transcoder::B, Pipe::B, 1, Protocol::Dp { lanes: 4 }, &t(true, false));
    assert_eq!(dp, (1 << 31) | (1 << 28) | (2 << 24) | (1 << 16) | (3 << 1));
    let edp = func_ctl(Transcoder::Edp, Pipe::A, 0, Protocol::Dp { lanes: 2 }, &t(false, true));
    assert_eq!(edp, (1 << 31) | (2 << 24) | (1 << 17) | (4 << 12) | (1 << 1));
    let hdmi = func_ctl(Transcoder::C, Pipe::C, 3, Protocol::Hdmi, &t(true, true));
    assert_eq!(hdmi, (1 << 31) | (3 << 28) | (3 << 16));
}

#[test]
fn a_transcoder_starts_and_stops_on_its_state_bit() {
    let mut m = Gen9::new(1);
    enable(&mut m, Transcoder::A, &Polls::new(20)).unwrap();
    assert_eq!(m.get(trans_conf(Transcoder::A)) & (3 << 30), 3 << 30);
    m.set(trans_ddi_func_ctl(Transcoder::A), 1 << 31);
    disable(&mut m, Transcoder::A, &Polls::new(20)).unwrap();
    assert_eq!(m.get(trans_conf(Transcoder::A)) & (3 << 30), 0);
    assert_eq!(m.get(trans_ddi_func_ctl(Transcoder::A)), 0);
    m.dead = true;
    assert_eq!(enable(&mut m, Transcoder::B, &Polls::new(20)), Err(Error::Timeout));
}
