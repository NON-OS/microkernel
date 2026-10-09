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
//! Codecs as their parameters describe them, for `sim` to answer with and for
//! the pure planner to read.
//!
//! The ALC236 is the codec in the HP Laptop 15s-fq0xxx (board 8651, Gemini
//! Lake Refresh) this driver is first meant to play on, from the layout
//! Linux codec dumps of that part show: two DACs (0x02, 0x03), no output
//! mixer, the speaker at 0x14 and the headphone jack at 0x21 each choosing
//! between the DACs, a digital microphone at 0x12 and S/PDIF at 0x1e. The
//! Intel display codec carries only HDMI and DisplayPort pins. The QEMU
//! duplex codec is what the driver was first tested on.

use crate::controller::codec::widget::{Codec, Widget};
use crate::controller::codec::pincfg::PinConfig;
use crate::sim::SimCodec;

pub const P_VENDOR: u16 = 0x00;
pub const P_SUBNODES: u16 = 0x04;
pub const P_FG_TYPE: u16 = 0x05;
pub const P_WCAPS: u16 = 0x09;
pub const P_PCM: u16 = 0x0a;
pub const P_PINCAP: u16 = 0x0c;
pub const P_AMP_IN: u16 = 0x0d;
pub const P_CONNLEN: u16 = 0x0e;
pub const P_GPIO: u16 = 0x11;
pub const P_AMP_OUT: u16 = 0x12;

pub const PCM_48K16: u32 = 0x000e_0560;
pub const DAC_WCAPS: u32 = 0x0000_041d;
pub const DAC_AMP: u32 = 0x0005_7057;
pub const PIN_OUT_WCAPS: u32 = 0x0040_058d;
pub const PIN_IN_WCAPS: u32 = 0x0040_048b;
pub const NC: u32 = 0x4111_11f0;
pub const HP_SUBSYSTEM: u32 = 0x103c_8651;

pub struct Desc {
    pub vendor: u32,
    pub subsystem: u32,
    pub gpio: u32,
    pub widgets: Vec<Widget>,
}

fn w(nid: u8, wcaps: u32) -> Widget {
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

pub fn mixer(nid: u8, conn: &[u8]) -> Widget {
    let mut m = w(nid, 0x0020_010b);
    m.amp_in = 0x8000_0000;
    set_conn(&mut m, conn);
    m
}

/// A beep generator widget (type 7) with an output amp.
pub fn beep(nid: u8) -> Widget {
    let mut b = w(nid, 0x0070_0004);
    b.amp_out = 0x8000_0000;
    b
}

pub fn selector(nid: u8, conn: &[u8]) -> Widget {
    let mut s = w(nid, 0x0030_0101);
    set_conn(&mut s, conn);
    s
}

fn set_conn(x: &mut Widget, conn: &[u8]) {
    for (i, &c) in conn.iter().enumerate() {
        x.conn[i] = c;
    }
    x.n_conn = conn.len() as u8;
}

pub fn alc236_hp() -> Desc {
    Desc {
        vendor: 0x10ec_0236,
        subsystem: HP_SUBSYSTEM,
        gpio: 0x4000_0003,
        widgets: vec![
            dac(0x02, PCM_48K16),
            dac(0x03, PCM_48K16),
            w(0x08, 0x0010_051b),
            w(0x09, 0x0010_051b),
            pin(0x12, 0x0040_040b, 0x0000_0020, 0x90a6_0120, &[]),
            pin(0x13, 0x0040_040b, 0x0000_0020, NC, &[]),
            pin(0x14, PIN_OUT_WCAPS, 0x0001_0014, 0x9017_0110, &[0x02, 0x03]),
            pin(0x18, PIN_IN_WCAPS, 0x0000_3724, NC, &[]),
            pin(0x19, PIN_IN_WCAPS, 0x0000_3724, 0x03a1_1030, &[]),
            pin(0x1a, PIN_OUT_WCAPS, 0x0000_3734, NC, &[0x02, 0x03]),
            pin(0x1b, PIN_OUT_WCAPS, 0x0001_3734, NC, &[0x02, 0x03]),
            pin(0x1d, 0x0040_0400, 0x0000_0020, 0x40e9_a105, &[]),
            pin(0x1e, 0x0040_0781, 0x0000_0014, NC, &[0x06]),
            w(0x20, 0x00f0_0040),
            pin(0x21, PIN_OUT_WCAPS, 0x0001_001c, 0x0221_1020, &[0x02, 0x03]),
        ],
    }
}

pub fn intel_hdmi() -> Desc {
    let mut cvt = dac(0x02, PCM_48K16);
    cvt.wcaps = 0x0000_6611;
    Desc {
        vendor: 0x8086_280d,
        subsystem: 0x8086_0101,
        gpio: 0,
        widgets: vec![
            cvt,
            pin(0x05, 0x0040_778d, 0x0b00_0094, 0x1856_0010, &[0x02]),
            pin(0x06, 0x0040_778d, 0x0b00_0094, 0x5856_0020, &[0x02]),
        ],
    }
}

pub fn qemu_duplex() -> Desc {
    let mut d = dac(0x02, 0x0002_0040);
    d.amp_out = 0x0003_4a4a;
    Desc {
        vendor: 0x1af4_0022,
        subsystem: 0x1af4_1100,
        gpio: 0,
        widgets: vec![d, pin(0x03, 0x0040_0101, 0x0000_0010, 0x0001_4010, &[0x02])],
    }
}

/// An older Realtek layout: each pin reaches its DAC through a mixer that
/// also sums the analog loopback 0x0b.
pub fn alc269_mixers() -> Desc {
    Desc {
        vendor: 0x10ec_0269,
        subsystem: 0x1025_0000,
        gpio: 0x4000_0002,
        widgets: vec![
            dac(0x02, PCM_48K16),
            dac(0x03, PCM_48K16),
            mixer(0x0b, &[0x18, 0x19]),
            mixer(0x0c, &[0x02, 0x0b]),
            mixer(0x0d, &[0x03, 0x0b]),
            pin(0x14, PIN_OUT_WCAPS, 0x0001_0014, 0x9017_0110, &[0x0c, 0x0d]),
            pin(0x15, PIN_OUT_WCAPS, 0x0001_001c, 0x0221_1020, &[0x0c, 0x0d]),
        ],
    }
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
