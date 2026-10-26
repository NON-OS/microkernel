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
//! The IDT (SigmaTel) port power map: the 92HD71, 73, 83, 89 and 95 power
//! each listed port down while its bit in the vendor verb 7ECh on the
//! function group is set. Linux clears the bit of every port whose default
//! configuration says something is connected and sets it for one that says
//! nothing is (`stac_init_power_map`, then `stac_init` writes the map;
//! sound/hda/codecs/sigmatel.c at 21a7f44fd076, :503-525 and :4366-4370,
//! the ports at :223-235 and :4650, the families at :5093-5128; values only,
//! no code taken). A jack is polled here, not watched, so a jack port stays
//! powered. Unverified on hardware.

use super::pincfg::CONN_NONE;
use super::widget::Codec;
use crate::controller::compose_verb;
use crate::controller::verb::Link;
use crate::error::HdaResult;

/// AC_VERB_IDT_SET_POWER_MAP.
pub const SET_POWER_MAP: u16 = 0x7ec;

const HD73: [u8; 8] = [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11];
const HD83: [u8; 7] = [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10];
const HD71: [u8; 3] = [0x0a, 0x0d, 0x0f];
const HD95: [u8; 4] = [0x0a, 0x0b, 0x0c, 0x0d];

/// The ports codec `vendor_id`'s power map covers, bit `i` for `[i]`.
pub const fn ports(vendor_id: u32) -> &'static [u8] {
    match vendor_id {
        0x111d_7674..=0x111d_7676 | 0x111d_76c0..=0x111d_76cb => &HD73,
        0x111d_7604 | 0x111d_76d4 | 0x111d_7605 | 0x111d_76d5 | 0x111d_76d1 | 0x111d_76d9 => &HD83,
        0x111d_7666..=0x111d_7669 => &HD83,
        0x111d_7603 | 0x111d_7608 | 0x111d_76b0..=0x111d_76b7 => &HD71,
        0x111d_7695 => &HD95,
        _ => &[],
    }
}

/// The map: a port's bit set (powered down) only when its pin says nothing
/// is connected; `None` for a codec without one.
pub fn power_map(codec: &Codec) -> Option<u8> {
    let ports = ports(codec.vendor_id);
    let none = |nid| codec.get(nid).is_some_and(|w| w.pin_cfg.connectivity() == CONN_NONE);
    let map = ports.iter().enumerate().filter(|&(_, &nid)| none(nid));
    (!ports.is_empty()).then(|| map.fold(0, |m, (i, _)| m | 1 << i))
}

pub fn init(link: &mut Link, codec: &Codec) -> HdaResult<()> {
    if let Some(map) = power_map(codec) {
        link.send(compose_verb(codec.cad, codec.afg, SET_POWER_MAP, map as u16))?;
    }
    Ok(())
}
