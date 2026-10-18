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

//! The external amplifier behind the codec: EAPD on every pin that has it
//! (`alc_auto_setup_eapd`), and on HP machines the GPIOs Linux's Realtek
//! fixups drive.

use super::program::hp_gpio;
use super::widget::Codec;
use crate::constants::{
    EAPD_ENABLE, PINCAP_EAPD, VERB_SET_EAPD_BTLENABLE, VERB_SET_GPIO_DATA, VERB_SET_GPIO_DIRECTION,
    VERB_SET_GPIO_MASK, WIDGET_TYPE_PIN,
};
use crate::controller::compose_verb;
use crate::controller::verb::Link;
use crate::error::HdaResult;

/// EAPD on every pin that has it, then the HP GPIOs: mask, direction, and
/// the data a millisecond later, as Linux writes them.
pub(super) fn enable(link: &mut Link, codec: &Codec) -> HdaResult<()> {
    let cad = codec.cad;
    for w in codec.all() {
        if w.ty() == WIDGET_TYPE_PIN && w.pin_caps & PINCAP_EAPD != 0 {
            link.send(compose_verb(cad, w.nid, VERB_SET_EAPD_BTLENABLE, EAPD_ENABLE as u16))?;
        }
    }
    if let Some(bits) = hp_gpio(codec.subsystem_id, codec.gpio_count) {
        let afg = codec.afg;
        link.send(compose_verb(cad, afg, VERB_SET_GPIO_MASK, bits as u16))?;
        link.send(compose_verb(cad, afg, VERB_SET_GPIO_DIRECTION, bits as u16))?;
        crate::clock::pause_ms(1);
        link.send(compose_verb(cad, afg, VERB_SET_GPIO_DATA, bits as u16))?;
    }
    Ok(())
}
