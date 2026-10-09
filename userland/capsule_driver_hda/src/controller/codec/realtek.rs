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

//! The Realtek initialisation Linux applies to every ALC codec of a given
//! type, whatever machine it is in (sound/pci/hda/patch_realtek.c).
//!
//! Two steps matter for sound out of a laptop's speaker and headphone jack:
//! `alc_fill_eapd_coef`, which on most ALC parts must clear a coefficient
//! bit before the EAPD pin control drives the external amplifier at all,
//! and `alc256_init`, which the ALC230/235/236/255/256/257 (the codecs on
//! most Intel and AMD laptops of the last eight years, the HP 15s ALC236
//! among them) need to bring the headphone amplifier out of its low power
//! state. Coefficients sit behind the vendor widget 0x20 (or the hidden
//! nodes 0x53 and 0x57) through SET_COEF_INDEX and GET/SET_PROC_COEF.
//!
//! Per-machine fixups (a GPIO for a particular board's amplifier, pin
//! overrides) are not here; `program::hp_gpio` covers the one generic HP
//! case.

use super::widget::Codec;
use crate::clock::pause_ms;
use crate::constants::{
    AMP_MUTE, AMP_OUT_UNMUTE, PIN_OUT_ENABLE, VERB_GET_PROC_COEF, VERB_SET_AMP_GAIN_MUTE,
    VERB_SET_COEF_INDEX, VERB_SET_PIN_WIDGET_CONTROL, VERB_SET_PROC_COEF,
};
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

pub const VENDOR_REALTEK: u16 = 0x10ec;
const COEF_NID: u8 = 0x20;

/// One read-modify-write of a coefficient: clear `mask`, set `set`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CoefUpdate {
    pub nid: u8,
    pub idx: u16,
    pub mask: u16,
    pub set: u16,
}

const fn upd(idx: u16, mask: u16, set: u16) -> CoefUpdate {
    CoefUpdate { nid: COEF_NID, idx, mask, set }
}

/// `alc_fill_eapd_coef` for codec `vendor_id`, whose coefficient 0 reads
/// `coef0`. At most three updates apply to one codec.
pub fn eapd_coef(vendor_id: u32, coef0: u16) -> [Option<CoefUpdate>; 3] {
    let variant = coef0 & 0x00f0;
    match vendor_id {
        0x10ec_0262 => [Some(upd(0x7, 0, 1 << 5)), None, None],
        0x10ec_0267 | 0x10ec_0268 => [Some(upd(0x7, 0, 1 << 13)), None, None],
        0x10ec_0269 => match variant {
            0x10 => [Some(upd(0xd, 0, 1 << 14)), None, None],
            0x20 => [Some(upd(0x4, 1 << 15, 0)), None, None],
            0x30 => [Some(upd(0x10, 1 << 9, 0)), None, None],
            _ => [None, None, None],
        },
        0x10ec_0280 | 0x10ec_0284 | 0x10ec_0290 | 0x10ec_0292 => {
            [Some(upd(0x4, 1 << 15, 0)), None, None]
        }
        0x10ec_0225 | 0x10ec_0295 | 0x10ec_0299 => [
            Some(upd(0x67, 0xf000, 0x3000)),
            Some(upd(0x36, 1 << 13, 0)),
            Some(upd(0x10, 1 << 9, 0)),
        ],
        0x10ec_0215 | 0x10ec_0285 | 0x10ec_0289 => {
            [Some(upd(0x36, 1 << 13, 0)), Some(upd(0x10, 1 << 9, 0)), None]
        }
        0x10ec_0230 | 0x10ec_0233 | 0x10ec_0235 | 0x10ec_0236 | 0x10ec_0245 | 0x10ec_0255
        | 0x10ec_0256 | 0x19e5_8326 | 0x10ec_0257 | 0x10ec_0282 | 0x10ec_0283 | 0x10ec_0286
        | 0x10ec_0288 | 0x10ec_0298 | 0x10ec_0300 => [Some(upd(0x10, 1 << 9, 0)), None, None],
        0x10ec_0275 => [Some(upd(0xe, 0, 1)), None, None],
        0x10ec_0287 => [Some(upd(0x10, 1 << 9, 0)), Some(upd(0x8, 0xffff, 0x4ab7)), None],
        0x10ec_0293 => [Some(upd(0xa, 1 << 13, 0)), None, None],
        0x10ec_0234 | 0x10ec_0274 | 0x10ec_0294 | 0x10ec_0700 | 0x10ec_0701 | 0x10ec_0703
        | 0x10ec_0711 => [Some(upd(0x10, 1 << 15, 0)), None, None],
        0x10ec_0662 if variant == 0x30 => [Some(upd(0x4, 1 << 10, 0)), None, None],
        0x10ec_0272 | 0x10ec_0273 | 0x10ec_0663 | 0x10ec_0665 | 0x10ec_0670 | 0x10ec_0671
        | 0x10ec_0672 => [Some(upd(0xd, 0, 1 << 14)), None, None],
        0x10ec_0222 | 0x10ec_0623 => [Some(upd(0x19, 1 << 13, 0)), None, None],
        0x10ec_0668 => [Some(upd(0x7, 3 << 13, 0)), None, None],
        0x10ec_0867 => [Some(upd(0x4, 1 << 10, 0)), None, None],
        0x10ec_0888 if variant == 0x20 || variant == 0x30 => [Some(upd(0x7, 1 << 5, 0)), None, None],
        0x10ec_0892 | 0x10ec_0897 => [Some(upd(0x7, 1 << 5, 0)), None, None],
        0x10ec_0899 | 0x10ec_0900 | 0x10ec_0b00 | 0x10ec_1168 | 0x10ec_1220 => {
            [Some(upd(0x7, 1 << 1, 0)), None, None]
        }
        _ => [None, None, None],
    }
}

/// The codecs `alc256_init` runs on (`patch_alc269`, the ALC255 and ALC256
/// types and the ALC257).
pub const fn uses_alc256_init(vendor_id: u32) -> bool {
    matches!(
        vendor_id,
        0x10ec_0230 | 0x10ec_0235 | 0x10ec_0236 | 0x10ec_0255 | 0x10ec_0256 | 0x19e5_8326 | 0x10ec_0257
    )
}

pub fn read_coef(link: &mut Link, cad: u8, nid: u8, idx: u16) -> HdaResult<u16> {
    link.send(compose_verb_long(cad, nid, VERB_SET_COEF_INDEX as u16, idx))?;
    Ok(link.send(compose_verb(cad, nid, VERB_GET_PROC_COEF, 0))? as u16)
}

pub fn write_coef(link: &mut Link, cad: u8, nid: u8, idx: u16, val: u16) -> HdaResult<()> {
    link.send(compose_verb_long(cad, nid, VERB_SET_COEF_INDEX as u16, idx))?;
    link.send(compose_verb_long(cad, nid, VERB_SET_PROC_COEF as u16, val))?;
    Ok(())
}

pub fn update(link: &mut Link, cad: u8, u: CoefUpdate) -> HdaResult<()> {
    let v = read_coef(link, cad, u.nid, u.idx)?;
    write_coef(link, cad, u.nid, u.idx, (v & !u.mask) | u.set)
}

/// `alc_fill_eapd_coef`, then `alc256_init` where it applies. `hp_pin` is
/// the headphone pin and `hp_plugged` what its jack sensed.
pub fn init(link: &mut Link, codec: &Codec, hp_pin: u8, hp_plugged: bool) -> HdaResult<()> {
    if (codec.vendor_id >> 16) as u16 != VENDOR_REALTEK && codec.vendor_id != 0x19e5_8326 {
        return Ok(());
    }
    let cad = codec.cad;
    let coef0 = read_coef(link, cad, COEF_NID, 0)?;
    for u in eapd_coef(codec.vendor_id, coef0).into_iter().flatten() {
        update(link, cad, u)?;
    }
    if uses_alc256_init(codec.vendor_id) {
        alc256_init(link, cad, hp_pin, hp_plugged)?;
    }
    Ok(())
}

fn alc256_init(link: &mut Link, cad: u8, hp: u8, sensed: bool) -> HdaResult<()> {
    let at = |nid, idx, mask, set| CoefUpdate { nid, idx, mask, set };
    pause_ms(30);
    if sensed {
        pause_ms(2);
    }
    update(link, cad, at(0x57, 0x04, 0x0007, 0x1))?;
    let mute = AMP_OUT_UNMUTE | AMP_MUTE;
    link.send(compose_verb_long(cad, hp, VERB_SET_AMP_GAIN_MUTE as u16, mute))?;
    if sensed {
        pause_ms(85);
    }
    link.send(compose_verb(cad, hp, VERB_SET_PIN_WIDGET_CONTROL, PIN_OUT_ENABLE as u16))?;
    if sensed {
        pause_ms(100);
    }
    update(link, cad, upd(0x46, 3 << 12, 0))?;
    update(link, cad, at(0x57, 0x04, 0x0007, 0x4))?;
    update(link, cad, at(0x53, 0x02, 0x8000, 0x8000))?;
    update(link, cad, at(0x53, 0x02, 0x8000, 0))?;
    write_coef(link, cad, COEF_NID, 0x36, 0x5757)
}
