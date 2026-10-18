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

//! Widgets as their parameters describe them: pins, converters, the beep
//! generator, and the connection list each may carry.

use super::{DAC_AMP, DAC_WCAPS};
use crate::controller::codec::pincfg::PinConfig;
use crate::controller::codec::widget::Widget;

pub(super) fn w(nid: u8, wcaps: u32) -> Widget {
    Widget::empty(nid).with_wcaps(wcaps)
}

trait Build {
    fn with_wcaps(self, v: u32) -> Self;
}

impl Build for Widget {
    fn with_wcaps(mut self, v: u32) -> Self {
        self.wcaps = v;
        self
    }
}

pub fn pin(nid: u8, wcaps: u32, caps: u32, cfg: u32, conn: &[u8]) -> Widget {
    let mut p = w(nid, wcaps);
    p.pin_caps = caps;
    p.pin_cfg = PinConfig(cfg);
    if p.has_out_amp() {
        p.amp_out = 0x8000_0000;
    }
    set_conn(&mut p, conn);
    p
}

pub fn dac(nid: u8, pcm: u32) -> Widget {
    let mut d = w(nid, DAC_WCAPS);
    d.amp_out = DAC_AMP;
    d.pcm = pcm;
    d
}

/// A beep generator widget (type 7) with an output amp.
pub fn beep(nid: u8) -> Widget {
    let mut b = w(nid, 0x0070_0004);
    b.amp_out = 0x8000_0000;
    b
}

pub(super) fn set_conn(x: &mut Widget, conn: &[u8]) {
    for (i, &c) in conn.iter().enumerate() {
        x.conn[i] = c;
    }
    x.n_conn = conn.len() as u8;
}
