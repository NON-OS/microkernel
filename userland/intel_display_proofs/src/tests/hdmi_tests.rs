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

//! HDMI clock and buffer bring-up, and the HDMI mode set.

use crate::engine::ddi::{buf_ctl, buf_trans, BUF_ENABLE};
use crate::engine::dpll::{cfgcr1, cfgcr2, DPLL_STATUS};
use crate::engine::dpll_cfgcr::cfgcr;
use crate::engine::dpll_hdmi::dco;
use crate::engine::error::Error;
use crate::engine::hdmi::bring_up;
use crate::engine::modeset_off::{darken, enable_hdmi};
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::regs::{trans_conf, trans_ddi_func_ctl};
use crate::model::{Gen9, Ticks};

fn lit_port() -> Gen9 {
    let mut m = Gen9::new(1);
    m.set(buf_trans(3, 0), 0x18);
    m.set(buf_ctl(3), 9 << 24);
    m
}

#[test]
fn the_dpll_runs_the_hdmi_clock_and_the_buffer_keeps_its_entry() {
    let mut m = lit_port();
    bring_up(&mut m, &Ticks::new(), 3, 2, 148_500, 0).unwrap();
    let (c1, c2) = cfgcr(&dco(148_500).unwrap()).unwrap();
    assert_eq!((m.get(cfgcr1(2)), m.get(cfgcr2(2))), (c1, c2));
    assert_ne!(m.get(DPLL_STATUS) & (1 << 16), 0);
    assert_eq!(m.get(buf_ctl(3)), BUF_ENABLE | (9 << 24));
}

#[test]
fn a_clock_past_the_sinks_tmds_limit_or_an_unlit_port_is_refused_untouched() {
    let mut m = lit_port();
    assert_eq!(
        bring_up(&mut m, &Ticks::new(), 3, 2, 297_000, 0),
        Err(Error::Argument),
        "165 MHz by default"
    );
    assert_eq!(
        bring_up(&mut m, &Ticks::new(), 3, 0, 148_500, 0),
        Err(Error::Argument),
        "never DPLL 0"
    );
    m.set(buf_trans(3, 0), 0);
    assert_eq!(bring_up(&mut m, &Ticks::new(), 3, 2, 148_500, 0), Err(Error::Layout));
    assert!(m.writes.is_empty());
}

#[test]
fn an_hdmi_output_lights_and_darkens_whole() {
    let mut m = lit_port();
    let t = nonos_modes::dmt(1920, 1080, 60, false).unwrap().timing;
    let out = (Transcoder::C, Pipe::C, 3, 2);
    enable_hdmi(&mut m, &Ticks::new(), out, &t, 340_000).unwrap();
    assert_eq!(m.get(trans_conf(Transcoder::C)) & (1 << 30), 1 << 30);
    assert_eq!(m.get(trans_ddi_func_ctl(Transcoder::C)) >> 24 & 7, 0, "HDMI mode");
    darken(&mut m, &Ticks::new(), out).unwrap();
    assert_eq!((m.get(trans_conf(Transcoder::C)), m.get(buf_ctl(3)) & BUF_ENABLE), (0, 0));
}
