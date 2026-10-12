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

//! The monitor's EDID read on the lit output's own port, and what the wire
//! then says: the panel's size in CAPS, the shown mode marked as one the
//! monitor lists. A silent monitor leaves both empty.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::mode::FROM_EDID;
use nonos_gfx_wire::Mode;

use super::edid_blob::edid_1080p;
use super::wire_rig::rig;
use crate::display::{panel, panel_of};
use crate::engine::aux_regs::Ddi;
use crate::engine::gen::{GEN11, GEN9};
use crate::engine::pipe::Transcoder;
use crate::engine::route::{Link, Route};
use crate::model::sink::sink;
use crate::model::{Gen9, Ticks};

#[test]
fn an_edp_panel_is_read_over_aux_and_its_size_reaches_caps() {
    let (mut d, model) = rig();
    model.borrow_mut().sink = Some((Ddi::A, sink(edid_1080p((344, 194)))));
    d.panel = panel_of(&mut d.m, &Ticks::new(), &GEN9, Transcoder::Edp);
    assert_eq!(d.panel.physical_mm, 344 | 194 << 16);
    assert_eq!(d.caps().physical_mm, 344 | 194 << 16);
    let mut out = [Mode::default(); 2];
    d.modes(0, &mut out).unwrap();
    assert_ne!(out[0].flags & FROM_EDID, 0, "the panel lists the mode shown");
}

#[test]
fn an_hdmi_monitor_is_read_over_ddc() {
    let mut m = Gen9::new(1);
    m.ddc.monitor = Some((5, edid_1080p((600, 340))));
    let p = panel(&mut m, &Ticks::new(), &Route { ddi: 1, link: Link::Hdmi });
    assert_eq!((p.preferred, p.physical_mm), (Some((1920, 1080)), 600 | 340 << 16));
}

#[test]
fn a_silent_monitor_or_another_generation_gives_an_empty_panel() {
    let (mut d, model) = rig();
    let p = panel_of(&mut d.m, &Ticks::new(), &GEN9, Transcoder::Edp);
    assert_eq!((p.physical_mm, p.len), (0, 0), "nothing behind AUX");
    model.borrow_mut().sink = Some((Ddi::A, sink(edid_1080p((344, 194)))));
    let p = panel_of(&mut d.m, &Ticks::new(), &GEN11, Transcoder::Edp);
    assert_eq!(p.len, 0, "Gen9's AUX registers are not read on Gen11");
}
