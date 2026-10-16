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

//! `alc256_init`, which brings the headphone amplifier of the ALC230/235/
//! 236/255/256/257 out of its low power state.

use super::realtek_coef::{upd, update, write_coef, CoefUpdate, COEF_NID};
use crate::clock::pause_ms;
use crate::constants::{
    AMP_MUTE, AMP_OUT_UNMUTE, PIN_OUT_ENABLE, VERB_SET_AMP_GAIN_MUTE, VERB_SET_PIN_WIDGET_CONTROL,
};
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

/// The codecs `alc256_init` runs on (`patch_alc269`, the ALC255 and ALC256
/// types and the ALC257).
pub const fn uses_alc256_init(vendor_id: u32) -> bool {
    matches!(
        vendor_id,
        0x10ec_0230
            | 0x10ec_0235
            | 0x10ec_0236
            | 0x10ec_0255
            | 0x10ec_0256
            | 0x19e5_8326
            | 0x10ec_0257
    )
}

pub(super) fn alc256_init(link: &mut Link, cad: u8, hp: u8, sensed: bool) -> HdaResult<()> {
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
