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
//! overrides) are not here; `program::hp_gpio`, through `vendor_gpio`,
//! covers the one generic HP case.

use super::realtek_alc256::{alc256_init, uses_alc256_init};
use super::realtek_coef::{read_coef, update, COEF_NID};
use super::realtek_eapd::eapd_coef;
use super::widget::Codec;
use crate::controller::verb::Link;
use crate::error::HdaResult;

pub const VENDOR_REALTEK: u16 = 0x10ec;

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
