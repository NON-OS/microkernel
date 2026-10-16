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

//! A codec description, and the two forms it is read in: a `Codec` for the
//! pure planner and a `SimCodec` for `sim` to answer with.

use super::{
    PCM_48K16, P_AMP_IN, P_AMP_OUT, P_CONNLEN, P_FG_TYPE, P_GPIO, P_PCM, P_PINCAP, P_SUBNODES,
    P_VENDOR, P_WCAPS,
};
use crate::controller::codec::widget::{Codec, Widget};
use crate::sim::SimCodec;

pub struct Desc {
    pub vendor: u32,
    pub subsystem: u32,
    pub gpio: u32,
    pub widgets: Vec<Widget>,
}

pub fn codec(d: &Desc, cad: u8) -> Box<Codec> {
    let mut c = Box::new(Codec::new(cad));
    c.vendor_id = d.vendor;
    c.subsystem_id = d.subsystem;
    c.afg = 1;
    c.gpio_count = d.gpio as u8;
    for x in &d.widgets {
        c.push(*x);
    }
    c
}

/// The same codec as `sim` answers it.
pub fn sim_codec(d: &Desc) -> SimCodec {
    let mut s = SimCodec { subsystem: d.subsystem, ..Default::default() };
    let first = d.widgets.iter().map(|w| w.nid).min().unwrap_or(2);
    let last = d.widgets.iter().map(|w| w.nid).max().unwrap_or(2);
    s.param(0, P_VENDOR, d.vendor).param(0, P_SUBNODES, (1 << 16) | 1);
    s.param(1, P_FG_TYPE, 1)
        .param(1, P_SUBNODES, ((first as u32) << 16) | (last - first + 1) as u32)
        .param(1, P_GPIO, d.gpio)
        .param(1, P_PCM, PCM_48K16)
        .param(1, P_AMP_OUT, 0)
        .param(1, P_AMP_IN, 0);
    for x in &d.widgets {
        let n = s.node(x.nid);
        n.params.insert(P_WCAPS, x.wcaps);
        n.params.insert(P_PINCAP, x.pin_caps);
        n.params.insert(P_AMP_IN, x.amp_in);
        n.params.insert(P_AMP_OUT, x.amp_out);
        n.params.insert(P_PCM, x.pcm);
        n.params.insert(P_CONNLEN, x.n_conn as u32);
        n.conn = x.connections().to_vec();
        n.pin_cfg = x.pin_cfg.0;
    }
    s
}
