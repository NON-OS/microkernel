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

//! How a lit transcoder reaches its monitor, read back from its DDI
//! function (SKL PRM Vol 2c, TRANS_DDI_FUNC_CTL; the encodings as
//! `trans_func` writes them, unverified there): which DDI, and whether it
//! speaks DisplayPort, HDMI or DVI. The eDP transcoder always drives DDI A
//! as DisplayPort. Read only, so a takeover can find the monitor's EDID
//! without touching the link.

use super::access::{field, Mmio};
use super::pipe::Transcoder;
use super::regs::trans_ddi_func_ctl;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Link {
    Dp { lanes: u32 },
    Hdmi,
    Dvi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Route {
    /// DDI A is 0.
    pub ddi: u32,
    pub link: Link,
}

/// The route of transcoder `t`, None when its function is off or in a
/// mode this driver does not drive (MST, FDI).
pub fn read(m: &mut dyn Mmio, t: Transcoder) -> Option<Route> {
    let ctl = m.read(trans_ddi_func_ctl(t));
    if ctl & (1 << 31) == 0 {
        return None;
    }
    let lanes = field(ctl, 3, 1) + 1;
    if t == Transcoder::Edp {
        return Some(Route { ddi: 0, link: Link::Dp { lanes } });
    }
    let ddi = field(ctl, 30, 28);
    let link = match field(ctl, 26, 24) {
        0 => Link::Hdmi,
        1 => Link::Dvi,
        2 => Link::Dp { lanes },
        _ => return None,
    };
    (1..=4).contains(&ddi).then_some(Route { ddi, link })
}
