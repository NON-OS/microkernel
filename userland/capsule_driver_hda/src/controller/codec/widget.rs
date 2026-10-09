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

//! What the walk learned about a codec, held as plain data.

use crate::constants::{
    PCM_BITS_16, PCM_RATE_48K, PINCAP_DP, PINCAP_HDMI, WCAP_DIGITAL, WCAP_IN_AMP, WCAP_OUT_AMP,
    WCAP_POWER, WIDGET_TYPE_PIN,
};

use super::pincfg::PinConfig;

/// Widgets per function group the walk records. A Realtek ALC236 has 0x23,
/// an Intel display codec about 0x10; a codec claiming more is cut here.
pub const MAX_WIDGETS: usize = 96;
/// Connections per widget recorded. Output paths run through the first few;
/// a long list belongs to an input mixer.
pub const MAX_CONN: usize = 24;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Widget {
    pub nid: u8,
    pub wcaps: u32,
    pub pin_caps: u32,
    pub pin_cfg: PinConfig,
    pub amp_in: u32,
    pub amp_out: u32,
    pub pcm: u32,
    pub conn: [u8; MAX_CONN],
    pub n_conn: u8,
}

impl Widget {
    pub const fn empty(nid: u8) -> Self {
        Widget {
            nid,
            wcaps: 0,
            pin_caps: 0,
            pin_cfg: PinConfig(0),
            amp_in: 0,
            amp_out: 0,
            pcm: 0,
            conn: [0; MAX_CONN],
            n_conn: 0,
        }
    }

    pub const fn ty(&self) -> u32 {
        (self.wcaps >> 20) & 0xf
    }

    pub const fn has_in_amp(&self) -> bool {
        self.wcaps & WCAP_IN_AMP != 0
    }

    pub const fn has_out_amp(&self) -> bool {
        self.wcaps & WCAP_OUT_AMP != 0
    }

    pub const fn has_power(&self) -> bool {
        self.wcaps & WCAP_POWER != 0
    }

    /// A digital widget, or a pin that carries HDMI or DisplayPort.
    pub const fn is_digital(&self) -> bool {
        if self.wcaps & WCAP_DIGITAL != 0 {
            return true;
        }
        self.ty() == WIDGET_TYPE_PIN && self.pin_caps & (PINCAP_HDMI | PINCAP_DP) != 0
    }

    /// A converter that takes the one format this driver plays.
    pub const fn plays_48k16(&self) -> bool {
        self.pcm & PCM_RATE_48K != 0 && self.pcm & PCM_BITS_16 != 0
    }

    pub fn connections(&self) -> &[u8] {
        &self.conn[..self.n_conn as usize]
    }
}

pub struct Codec {
    pub cad: u8,
    pub vendor_id: u32,
    pub subsystem_id: u32,
    pub afg: u8,
    pub gpio_count: u8,
    pub widgets: [Widget; MAX_WIDGETS],
    pub n: usize,
}

impl Codec {
    pub const fn new(cad: u8) -> Self {
        Codec {
            cad,
            vendor_id: 0,
            subsystem_id: 0,
            afg: 0,
            gpio_count: 0,
            widgets: [Widget::empty(0); MAX_WIDGETS],
            n: 0,
        }
    }

    pub fn get(&self, nid: u8) -> Option<&Widget> {
        self.widgets[..self.n].iter().find(|w| w.nid == nid)
    }

    pub fn all(&self) -> &[Widget] {
        &self.widgets[..self.n]
    }

    /// Append a widget; false when the description is full.
    pub fn push(&mut self, w: Widget) -> bool {
        if self.n >= MAX_WIDGETS {
            return false;
        }
        self.widgets[self.n] = w;
        self.n += 1;
        true
    }
}
